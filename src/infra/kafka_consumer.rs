use crate::config::KafkaConfig;
use crate::errors::Result;
use rdkafka::consumer::StreamConsumer;
use rdkafka::ClientConfig;
use tokio::sync::mpsc;
use tracing::info;

pub struct KafkaConsumer {
    _consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(config: KafkaConfig, _symbol_tx: mpsc::Sender<Vec<String>>) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("group.id", &config.consumer.group_id)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .create()?;

        info!(
            component = "kafka_consumer",
            "Created Kafka consumer with group_id: {}",
            config.consumer.group_id
        );

        // TODO: Subscribe to symbol management topics and forward updates to symbol_tx
        // For now, this is a placeholder that creates the consumer but doesn't actively consume

        Ok(Self {
            _consumer: consumer,
        })
    }
}
