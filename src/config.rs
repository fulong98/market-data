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
    #[serde(default)]
    pub shutdown: ShutdownConfig,
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
    #[serde(default = "default_max_reconnect_attempts")]
    pub max_reconnect_attempts: u32,
    #[serde(default = "default_initial_backoff_ms")]
    pub initial_backoff_ms: u64,
    #[serde(default = "default_send_timeout_ms")]
    pub send_timeout_ms: u64,
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

fn default_max_reconnect_attempts() -> u32 {
    3
}

fn default_initial_backoff_ms() -> u64 {
    1000
}

fn default_send_timeout_ms() -> u64 {
    100
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    #[serde(default = "default_max_reconnect_attempts")]
    pub max_reconnect_attempts: u32,
    #[serde(default = "default_initial_backoff_ms")]
    pub initial_backoff_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickhouseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    #[serde(default = "default_max_reconnect_attempts")]
    pub max_reconnect_attempts: u32,
    #[serde(default = "default_initial_backoff_ms")]
    pub initial_backoff_ms: u64,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShutdownConfig {
    #[serde(default = "default_total_timeout_ms")]
    pub total_timeout_ms: u64,
    #[serde(default = "default_task_join_timeout_ms")]
    pub task_join_timeout_ms: u64,
    #[serde(default = "default_kafka_flush_timeout_ms")]
    pub kafka_flush_timeout_ms: u64,
    #[serde(default = "default_redis_drain_timeout_ms")]
    pub redis_drain_timeout_ms: u64,
    #[serde(default = "default_exchange_unsubscribe_timeout_ms")]
    pub exchange_unsubscribe_timeout_ms: u64,
}

fn default_total_timeout_ms() -> u64 {
    5000
}

fn default_task_join_timeout_ms() -> u64 {
    2500
}

fn default_kafka_flush_timeout_ms() -> u64 {
    2000
}

fn default_redis_drain_timeout_ms() -> u64 {
    300
}

fn default_exchange_unsubscribe_timeout_ms() -> u64 {
    1000
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
