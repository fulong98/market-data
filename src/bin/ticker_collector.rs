use market_data::errors::Result;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    info!(component="ticker_collector", "Starting...");

    // TODO: Implement Config, KafkaConsumer, ExchangeFactory, and health_check modules
    // let config = Config::load()?;

    // let (symbol_tx, symbol_rx) = mpsc::channel(100);

    // let kafka_producer = KafkaProducer::new(config.kafka);
    // let kafka_consumer = KafkaConsumer::new(config.kafka, symbol_tx);

    // let redis_storage = RedisStorage::new(&config.redis);

    // tokio::spawn(async move {
    //     if let Err(e) = health_check::start_server(config.health_check.port).await{
    //         error!("Health check server failed: {}",e);
    //     }
    // });
    // TODO: cancellation token

    // let exchange_factory = ExchangeFactory::new(&config.exchange, symbol_rx);

    // for exchange_config in config.exchanges.values() {
    //     if !exchange_config.enabled{
    //         continue;
    //     }

    //     let exchange = exchange_factory.create_exchange(&exchange_config.name)?;
    //     exchange.subscribe(&config.symbols)?;
    //     let mut ticker_stream = exchange.connect_ticker().await;
    //     tokio::spawn(async move{
    //         while let Some(result) = ticker_stream.next().await{
    //             match result {
    //                 Ok(market_data) => {
    //                     if let Err(e) = redis_storage.update_latest_data(&market_data).await{
    //                         error!("Failed to update Redis: {}",e)
    //                     }

    //                     if let Err(e) = kafka_producer.send_market_data(&market_data).await{
    //                         error!("Failed to send to kafka:{}", e);
    //                     }
    //                 }
    //                 Err(e) => {
    //                     error!("Error receiving market data: {}", e);
    //                     // implement retry mechanism or circuit breaker
    //                 }
    //             }
    //         }
    //     });
    // }

    info!("Ticker collector started succesfully");

    signal::ctrl_c().await?;
    info!("Shutting down Ticker collector..");

    Ok(())



}