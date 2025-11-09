use crate::config::Config;
use crate::errors::{MarketDataError, Result};
use crate::exchanges::deribit::models::Exchange;
use crate::exchanges::deribit::{Deribit, DeribitConfig};
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct ExchangeFactory {
    config: Arc<Config>,
}

impl ExchangeFactory {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn create_exchange(
        &self,
        name: &str,
        symbol_rx: mpsc::Receiver<Vec<String>>,
    ) -> Result<Box<dyn Exchange>> {
        match name.to_lowercase().as_str() {
            "deribit" => {
                let exchange_config = self
                    .config
                    .exchanges
                    .get("deribit")
                    .ok_or_else(|| MarketDataError::ConfigError("Deribit config not found".to_string()))?;

                // Use default Deribit configuration
                let deribit_config = DeribitConfig::default();
                let mut deribit = Deribit::new(deribit_config, symbol_rx).await?;

                // Subscribe to configured symbols if any
                if !exchange_config.symbols.is_empty() {
                    deribit.subscribe(&exchange_config.symbols).await?;
                }

                Ok(Box::new(deribit))
            }
            _ => Err(MarketDataError::ExchangeNotSupported(format!(
                "Exchange '{}' is not supported",
                name
            ))),
        }
    }
}
