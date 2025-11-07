use thiserror::Error;

#[derive(Error, Debug)]
pub enum MarketDataError{
    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Websocket error: {0}")]
    WebSocketError(String),

    #[error("Kafka error:{0}")]
    KafkaError(#[from] rdkafka::error::KafkaError),

    #[error("Redis error: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("Clickhouse error: {0}")]
    ClickhouseError(#[from] clickhouse_rs::errors::Error),

    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Exchange not supported: {0}")]
    ExchangeNotSupported(String),
}

pub type Result<T> = std::result::Result<T, MarketDataError>;