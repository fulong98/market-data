use market_data::config::Config;
use market_data::errors::Result;
use market_data::exchanges::ExchangeFactory;
use market_data::health_check;
use market_data::infra::{KafkaConsumer, KafkaProducer, RedisStorage};
use futures::future::join_all;
use futures::StreamExt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::signal;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!(component="trades_collector", "Starting...");

    let config = Arc::new(Config::load()?);

    let (symbol_tx, _symbol_rx) = mpsc::channel(100);

    let kafka_producer = Arc::new(KafkaProducer::new(config.kafka.clone())?);
    let _kafka_consumer = KafkaConsumer::new(config.kafka.clone(), symbol_tx)?;

    let redis_storage = Arc::new(RedisStorage::new(&config.redis).await?);

    // Create cancellation token for graceful shutdown
    let shutdown_token = CancellationToken::new();

    // Spawn health check server with graceful shutdown
    let health_port = config.health_check.port;
    let health_shutdown = shutdown_token.clone();
    let health_handle: JoinHandle<()> = tokio::spawn(async move {
        if let Err(e) = health_check::start_server(
            health_port,
            async move { health_shutdown.cancelled().await },
        )
        .await
        {
            error!("Health check server failed: {}", e);
        }
    });

    let exchange_factory = ExchangeFactory::new(config.clone());
    let mut task_handles: Vec<JoinHandle<()>> = vec![health_handle];

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
        let task_token = shutdown_token.child_token();
        let exchange_name = exchange_name.clone();

        let handle = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = task_token.cancelled() => {
                        info!("Trades processor for {} shutting down", exchange_name);
                        break;
                    }
                    result = trades_stream.next() => {
                        match result {
                            Some(Ok(market_data)) => {
                                if let Err(e) = redis_storage.update_latest_data(&market_data).await {
                                    error!("Failed to update Redis: {}", e);
                                }

                                if let Err(e) = kafka_producer.send_market_data(&market_data).await {
                                    error!("Failed to send to kafka: {}", e);
                                }
                            }
                            Some(Err(e)) => {
                                error!("Error receiving market data: {}", e);
                            }
                            None => {
                                info!("Trades stream ended for {}", exchange_name);
                                break;
                            }
                        }
                    }
                }
            }
        });

        task_handles.push(handle);
    }

    info!("Trades collector started successfully");

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("SIGINT received, initiating graceful shutdown");
        }
        _ = async {
            let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to register SIGTERM handler");
            sigterm.recv().await
        } => {
            info!("SIGTERM received, initiating graceful shutdown");
        }
    }

    let shutdown_start = Instant::now();

    // Phase 1: Cancel all tasks
    info!("Phase 1: Cancelling all tasks");
    shutdown_token.cancel();

    // Phase 2: Wait for tasks to complete (with timeout)
    info!("Phase 2: Waiting for tasks to join (timeout: {}ms)", config.shutdown.task_join_timeout_ms);
    let task_join_timeout = Duration::from_millis(config.shutdown.task_join_timeout_ms);
    match tokio::time::timeout(task_join_timeout, join_all(task_handles)).await {
        Ok(_) => info!("All tasks joined successfully"),
        Err(_) => warn!("Task join timeout exceeded"),
    }

    // Phase 3: Flush Kafka
    info!("Phase 3: Flushing Kafka (timeout: {}ms)", config.shutdown.kafka_flush_timeout_ms);
    let kafka_flush_timeout = Duration::from_millis(config.shutdown.kafka_flush_timeout_ms);
    if let Err(e) = kafka_producer.flush(kafka_flush_timeout).await {
        error!("Kafka flush failed: {}", e);
    }

    // Phase 4: Drain Redis
    info!("Phase 4: Draining Redis (timeout: {}ms)", config.shutdown.redis_drain_timeout_ms);
    let redis_drain_timeout = Duration::from_millis(config.shutdown.redis_drain_timeout_ms);
    redis_storage.shutdown(redis_drain_timeout).await;

    let total_shutdown_time = shutdown_start.elapsed();
    info!(
        "Shutdown complete in {:?} (budget: {}ms)",
        total_shutdown_time,
        config.shutdown.total_timeout_ms
    );

    Ok(())
}
