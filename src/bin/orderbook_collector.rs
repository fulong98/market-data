use market_data::config::Config;
use market_data::errors::Result;
use market_data::exchanges::ExchangeFactory;
use market_data::health_check;
use market_data::infra::{KafkaProducer, RedisStorage};
use futures::StreamExt;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::mpsc;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Initializing logger...");
    tracing_subscriber::fmt::init();
    println!("Logger initialized, starting collector...");
    info!(component="orderbook_collector", "Starting...");

    info!("Loading configuration from config/default.toml");
    let config = match Config::load() {
        Ok(cfg) => {
            info!("Configuration loaded successfully");
            info!("Exchanges configured: {:?}", cfg.exchanges.keys().collect::<Vec<_>>());
            Arc::new(cfg)
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e);
        }
    };

    let kafka_producer = KafkaProducer::new(config.kafka.clone())?;
    let redis_storage = RedisStorage::new(&config.redis.url)?;

    // Spawn health check server
    let health_port = config.health_check.port;
    tokio::spawn(async move {
        if let Err(e) = health_check::start_server(health_port).await {
            error!("Health check server failed: {}", e);
        }
    });

    let exchange_factory = ExchangeFactory::new(config.clone());

    // Spawn a task for each enabled exchange
    for (exchange_name, exchange_config) in config.exchanges.iter() {
        info!("Processing exchange: {}", exchange_name);
        if !exchange_config.enabled {
            info!("Exchange {} is disabled, skipping", exchange_name);
            continue;
        }

        // Create a new symbol channel for each exchange
        let (_exchange_symbol_tx, exchange_symbol_rx) = mpsc::channel(100);

        info!("Creating exchange instance for: {}", exchange_name);
        let mut exchange = exchange_factory.create_exchange(exchange_name, exchange_symbol_rx).await?;

        info!("Connecting to orderbook stream for: {}", exchange_name);
        let mut orderbook_stream = exchange.connect_orderbook().await?;
        info!("Orderbook stream connected successfully");

        let kafka_producer = kafka_producer.clone();
        let redis_storage = redis_storage.clone();

        tokio::spawn(async move {
            while let Some(result) = orderbook_stream.next().await {
                match result {
                    Ok(market_data) => {
                        info!("Received market data: {:?}", market_data);

                        if let Err(e) = redis_storage.update_latest_data(&market_data).await {
                            error!("Failed to update Redis: {}", e);
                        }

                        if let Err(e) = kafka_producer.send_market_data(&market_data).await {
                            error!("Failed to send to kafka: {}", e);
                        }
                    }
                    Err(e) => {
                        error!("Error receiving market data: {}", e);
                        // TODO: implement retry mechanism or circuit breaker
                    }
                }
            }
        });
    }

    info!("Orderbook collector started successfully");

    signal::ctrl_c().await?;
    info!("Shutting down orderbook collector..");

    Ok(())



}