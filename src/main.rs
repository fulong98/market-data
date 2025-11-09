use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tracing::{info, error};

#[derive(Serialize, Deserialize, Debug)]
struct LogEvent {
    timestamp: String,
    level: String,
    message: String,
    service: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("Starting application...");
    
    // Initialize Kafka producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("message.timeout.ms", "5000")
        .create()?;
    
    // Initialize ClickHouse connection
    // TODO: Fix ClickHouse API usage - get_handle() doesn't return execute/insert methods
    // let pool = Pool::new("tcp://default:password@localhost:9000/default");
    // init_clickhouse_table(&pool).await?;

    // Example: Send a message to Kafka
    let event = LogEvent {
        timestamp: {
            use time::{OffsetDateTime, format_description::well_known::Rfc3339};
            OffsetDateTime::now_utc().format(&Rfc3339).unwrap_or_else(|_| String::from("unknown"))
        },
        level: "INFO".to_string(),
        message: "Application started".to_string(),
        service: "rust-app".to_string(),
    };
    
    let json_event = serde_json::to_string(&event)?;
    
    let delivery_status = producer.send(
        FutureRecord::to("logs-topic")
            .payload(&json_event)
            .key("log"),
        Duration::from_secs(0),
    ).await;
    
    match delivery_status {
        Ok(_) => info!("Message sent to Kafka"),
        Err((e, _)) => error!("Error sending message: {}", e),
    }
    
    // Store in ClickHouse
    // store_in_clickhouse(&pool, &event).await?;

    Ok(())
}

// TODO: Fix ClickHouse API usage
// async fn init_clickhouse_table(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = pool.get_handle();
//
//     client.execute("CREATE TABLE IF NOT EXISTS logs (timestamp DateTime64(3), level String, message String, service String) ENGINE = MergeTree() PARTITION BY toYYYYMM(timestamp) ORDER BY (timestamp, service)").await?;
//
//     Ok(())
// }
//
// async fn store_in_clickhouse(pool: &Pool, event: &LogEvent) -> Result<(), Box<dyn std::error::Error>> {
//     let mut client = pool.get_handle();
//
//     let mut insert = client.insert("logs")?;
//     insert
//         .write(&event.timestamp)
//         .write(&event.level)
//         .write(&event.message)
//         .write(&event.service)
//         .end()?;
//
//     Ok(())
// }