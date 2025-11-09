use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use crate::config::KafkaConfig;
use crate::errors::{Result, MarketDataError};
use crate::exchanges::deribit::models::MarketData;
use tracing::{debug, error};

#[derive(Clone)]
pub struct KafkaProducer {
    client: FutureProducer,
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
        let client: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("message.timeout.ms", &config.producer.timeout_ms.to_string())
            .create()
            .map_err(|e| MarketDataError::KafkaError(e))?;

        Ok(Self {
            client,
            orderbook_topic: config.orderbook_topic,
            trade_topic: config.trade_topic,
            ticker_topic: config.ticker_topic,
        })
    }

    pub async fn send_market_data(&self, data: &MarketData) -> Result<()> {
        let (topic, key) = match data {
            MarketData::Orderbook(ob) => {
                (&self.orderbook_topic, format!("{}.{}", ob.exchange, ob.instrument_name))
            }
            MarketData::Trade(trade) => {
                (&self.trade_topic, format!("{}.{}", trade.exchange, trade.instrument_name))
            }
            MarketData::Ticker(ticker) => {
                (&self.ticker_topic, format!("{}.{}", ticker.exchange, ticker.instrument_name))
            }
        };

        let json_data = serde_json::to_string(data)
            .map_err(|e| MarketDataError::JsonError(e))?;

        let record = FutureRecord::to(topic)
            .key(&key)
            .payload(&json_data);

        match self.client.send(record, Duration::from_secs(0)).await {
            Ok(_) => {
                debug!("Successfully sent market data to {}", topic);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send market data: {}", e);
                Err(MarketDataError::KafkaError(e))
            }
        }
    }
}