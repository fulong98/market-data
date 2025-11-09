use redis::{Client, Commands};
use crate::errors::Result;
use crate::exchanges::deribit::models::MarketData;
use tracing::info;

#[derive(Clone)]
pub struct RedisStorage {
    client: Client,
}

impl RedisStorage {
    pub fn new(url: &str) -> Result<Self> {
        let client = Client::open(url)?;
        Ok(Self { client })
    }

    pub async fn update_latest_data(&self, data: &MarketData) -> Result<()> {
        let mut con = self.client.get_connection()?;

        match data {
            MarketData::Orderbook(ob) => {
                // key: <exchange>:<instrument_name>:orderbook
                let key = format!("{}:{}:orderbook", ob.exchange, ob.instrument_name);
                let value = serde_json::to_string(ob)?;
                let _: () = con.set_ex(&key, value, 3)?; // 3 secs TTL
            }
            MarketData::Trade(trade) => {
                // key: <exchange>:<instrument_name>:last_trade
                let key = format!("{}:{}:last_trade", trade.exchange, trade.instrument_name);
                let value = serde_json::to_string(trade)?;
                let _: () = con.set_ex(&key, value, 60)?; // 1 minute TTL
            }
            MarketData::Ticker(ticker) => {
                // key: <exchange>:<instrument_name>:ticker
                let key = format!("{}:{}:ticker", ticker.exchange, ticker.instrument_name);
                let value = serde_json::to_string(ticker)?;
                let _: () = con.set_ex(&key, value, 300)?; // 5 min TTL
            }
        }

        info!(component = "redis", "Updated Redis with latest market data");
        Ok(())
    }
}