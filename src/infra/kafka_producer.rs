use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use crate::config::KafkaConfig;
use crate::errors::{Result, MarketDataError};
use crate::exchanges::deribit::models::MarketData;
use tracing::{debug, error, warn, info};
use tokio::time::sleep;

#[derive(Clone)]
pub struct KafkaProducer {
    client: FutureProducer,
    #[allow(dead_code)]
    config: KafkaConfig, // Kept for future reconnection logic
    orderbook_topic: String,
    trade_topic: String,
    ticker_topic: String,
}

pub struct KafkaProducerConfig {
    pub brokers: String,
    pub orderbook_topic: String,
    pub trade_topic: String,
    pub ticker_topic: String,
}

impl KafkaProducer {
    pub fn new(config: KafkaConfig) -> Result<Self> {
        let client = Self::create_producer(&config)?;

        info!(
            component = "kafka",
            brokers = %config.bootstrap_servers,
            "Created Kafka producer with optimized config"
        );

        Ok(Self {
            client,
            orderbook_topic: config.orderbook_topic.clone(),
            trade_topic: config.trade_topic.clone(),
            ticker_topic: config.ticker_topic.clone(),
            config,
        })
    }

    /// Create a new Kafka producer with optimized settings
    fn create_producer(config: &KafkaConfig) -> Result<FutureProducer> {
        ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("message.timeout.ms", &config.producer.timeout_ms.to_string())
            // Batching configuration for high throughput
            .set("linger.ms", "10")                     // Wait up to 10ms to batch messages
            .set("batch.size", "65536")                 // 64KB batches
            .set("compression.type", "lz4")             // Fast compression
            // Buffer configuration
            .set("queue.buffering.max.messages", "1000000")  // 1M messages
            .set("queue.buffering.max.kbytes", "1048576")    // 1GB buffer
            // Performance tuning
            .set("acks", "all")                         // Required for idempotence
            .set("retries", "3")                        // Retry up to 3 times
            .set("max.in.flight.requests.per.connection", "5")  // Pipeline requests
            // Enable idempotence for exactly-once semantics within retry window
            .set("enable.idempotence", "true")
            .create()
            .map_err(|e| MarketDataError::KafkaError(e))
    }

    /// Send market data with auto-reconnection on failure
    pub async fn send_market_data(&self, data: &MarketData) -> Result<()> {
        let mut attempts = 0;
        let mut backoff = self.config.producer.initial_backoff_ms;
        let max_attempts = self.config.producer.max_reconnect_attempts;

        loop {
            match self.try_send_market_data(data).await {
                Ok(_) => {
                    if attempts > 0 {
                        info!(component = "kafka", attempts, "Kafka send recovered");
                    }
                    return Ok(());
                }
                Err(e) => {
                    attempts += 1;

                    if attempts > max_attempts {
                        error!(
                            component = "kafka",
                            attempts,
                            error = %e,
                            "Failed to send to Kafka after {} attempts - PANICKING",
                            max_attempts
                        );
                        panic!(
                            "Kafka send failed after {} attempts. Error: {}. Pod will restart.",
                            max_attempts, e
                        );
                    }

                    warn!(
                        component = "kafka",
                        attempt = attempts,
                        backoff_ms = backoff,
                        error = %e,
                        "Kafka send failed, retrying..."
                    );

                    sleep(Duration::from_millis(backoff)).await;
                    backoff *= 2; // Exponential backoff
                }
            }
        }
    }

    /// Try to send data once (may fail if broker is down)
    async fn try_send_market_data(&self, data: &MarketData) -> Result<()> {
        let (topic, key) = match data {
            MarketData::Orderbook(ob) => {
                (&self.orderbook_topic, format!("{}.{}", ob.venue, ob.symbol))
            }
            MarketData::Trade(trade) => {
                (&self.trade_topic, format!("{}.{}", trade.venue, trade.symbol))
            }
            MarketData::Ticker(ticker) => {
                (&self.ticker_topic, format!("{}.{}", ticker.venue, ticker.symbol))
            }
        };

        let json_data = serde_json::to_string(data)
            .map_err(|e| MarketDataError::JsonError(e))?;

        let record = FutureRecord::to(topic)
            .key(&key)
            .payload(&json_data);

        let timeout = Duration::from_millis(self.config.producer.send_timeout_ms);
        match self.client.send(record, timeout).await {
            Ok(_) => {
                debug!("Successfully sent market data to {}", topic);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send market data to {}: {}", topic, e);
                Err(MarketDataError::KafkaError(e))
            }
        }
    }

    /// Get statistics about the Kafka producer
    pub fn statistics(&self) -> String {
        // This would return producer stats if rdkafka provides them
        // For now, return a placeholder
        "Kafka producer statistics not yet implemented".to_string()
    }
}
