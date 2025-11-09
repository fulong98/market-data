use market_data::config::Config;
use market_data::errors::Result;
use market_data::exchanges::ExchangeFactory;
use market_data::health_check;
use market_data::infra::{KafkaConsumer, KafkaProducer, RedisStorage};
use futures::StreamExt;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::mpsc;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!(component="trades_collector", "Starting...");

    let config = Arc::new(Config::load()?);

    let (symbol_tx, _symbol_rx) = mpsc::channel(100);

    let kafka_producer = KafkaProducer::new(config.kafka.clone())?;
    let _kafka_consumer = KafkaConsumer::new(config.kafka.clone(), symbol_tx)?;

    let redis_storage = RedisStorage::new(&config.redis).await?;

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
        if !exchange_config.enabled {
            continue;
        }

        // Create a new symbol channel for each exchange
        let (_exchange_symbol_tx, exchange_symbol_rx) = mpsc::channel(100);

        let mut exchange = exchange_factory.create_exchange(exchange_name, exchange_symbol_rx).await?;
        let mut trades_stream = exchange.connect_trades().await?;

        let kafka_producer = kafka_producer.clone();
        let redis_storage = redis_storage.clone();

        tokio::spawn(async move {
            while let Some(result) = trades_stream.next().await {
                match result {
                    Ok(market_data) => {
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

    info!("Trades collector started successfully");

    signal::ctrl_c().await?;
    info!("Shutting down trades collector..");

    Ok(())
}