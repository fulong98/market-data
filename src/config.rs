use crate::errors::{MarketDataError, Result};
use config::{Config as ConfigLoader, File};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub kafka: KafkaConfig,
    pub redis: RedisConfig,
    pub clickhouse: ClickhouseConfig,
    pub logging: LoggingConfig,
    pub health_check: HealthCheckConfig,
    pub exchanges: HashMap<String, ExchangeConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub orderbook_topic: String,
    pub trade_topic: String,
    pub ticker_topic: String,
    #[serde(default)]
    pub producer: KafkaProducerConfig,
    #[serde(default)]
    pub consumer: KafkaConsumerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KafkaProducerConfig {
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KafkaConsumerConfig {
    #[serde(default = "default_group_id")]
    pub group_id: String,
}

fn default_timeout() -> u64 {
    5000
}

fn default_group_id() -> String {
    "market-data-collectors".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickhouseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub port: u16,
    pub endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeConfig {
    pub enabled: bool,
    #[serde(default)]
    pub symbols: Vec<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config = ConfigLoader::builder()
            .add_source(File::with_name("config/default"))
            .build()
            .map_err(|e| MarketDataError::ConfigError(format!("Failed to load config: {}", e)))?;

        config
            .try_deserialize()
            .map_err(|e| MarketDataError::ConfigError(format!("Failed to parse config: {}", e)))
    }
}
