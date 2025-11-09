use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Client};
use crate::config::RedisConfig;
use crate::errors::Result;
use crate::exchanges::deribit::models::MarketData;
use tracing::{info, warn, error};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct RedisStorage {
    manager: ConnectionManager,
    url: String,
    config: RedisConfig,
}

impl RedisStorage {
    pub async fn new(config: &RedisConfig) -> Result<Self> {
        let client = Client::open(config.url.as_str())?;
        let manager = ConnectionManager::new(client).await?;

        info!(component = "redis", "Connected to Redis at {}", config.url);

        Ok(Self {
            manager,
            url: config.url.clone(),
            config: config.clone(),
        })
    }

    /// Update market data with auto-reconnection on failure
    pub async fn update_latest_data(&self, data: &MarketData) -> Result<()> {
        let mut attempts = 0;
        let mut backoff = self.config.initial_backoff_ms;
        let max_attempts = self.config.max_reconnect_attempts;

        loop {
            match self.try_update_latest_data(data).await {
                Ok(_) => {
                    if attempts > 0 {
                        info!(component = "redis", attempts, "Reconnected successfully");
                    }
                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;

                    if attempts > max_attempts {
                        error!(
                            component = "redis",
                            attempts,
                            error = %e,
                            "Failed to reconnect to Redis after {} attempts - PANICKING",
                            max_attempts
                        );
                        panic!(
                            "Redis reconnection failed after {} attempts. Error: {}. Pod will restart.",
                            max_attempts, e
                        );
                    }

                    warn!(
                        component = "redis",
                        attempt = attempts,
                        backoff_ms = backoff,
                        error = %e,
                        "Redis operation failed, retrying..."
                    );

                    sleep(Duration::from_millis(backoff)).await;
                    backoff *= 2; // Exponential backoff
                }
            }
        }
    }

    /// Try to update data once (may fail if connection is broken)
    async fn try_update_latest_data(&self, data: &MarketData) -> Result<()> {
        let mut con = self.manager.clone();

        match data {
            MarketData::Orderbook(ob) => {
                // key: <venue>:<symbol>:orderbook
                let key = format!("{}:{}:orderbook", ob.venue, ob.symbol);
                let value = serde_json::to_string(ob)?;
                con.set_ex::<_, _, ()>(&key, value, 3).await?; // 3 secs TTL
            }
            MarketData::Trade(trade) => {
                // key: <venue>:<symbol>:last_trade
                let key = format!("{}:{}:last_trade", trade.venue, trade.symbol);
                let value = serde_json::to_string(trade)?;
                con.set_ex::<_, _, ()>(&key, value, 60).await?; // 1 minute TTL
            }
            MarketData::Ticker(ticker) => {
                // key: <venue>:<symbol>:ticker
                let key = format!("{}:{}:ticker", ticker.venue, ticker.symbol);
                let value = serde_json::to_string(ticker)?;
                con.set_ex::<_, _, ()>(&key, value, 300).await?; // 5 min TTL
            }
        }

        info!(component = "redis", "Updated Redis with latest market data");
        Ok(())
    }

    /// Manually trigger reconnection (useful for testing or external health checks)
    pub async fn reconnect(&mut self) -> Result<()> {
        info!(component = "redis", "Manually reconnecting to Redis");
        let client = Client::open(self.url.as_str())?;
        self.manager = ConnectionManager::new(client).await?;
        info!(component = "redis", "Reconnected to Redis");
        Ok(())
    }
}
