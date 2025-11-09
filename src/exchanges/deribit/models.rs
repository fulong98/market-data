use serde::{Deserialize, Serialize};
use crate::errors::Result;
use futures::stream::BoxStream;
use async_trait::async_trait;
use time::OffsetDateTime;

// Unified Kafka schema for market data
// All data types include common metadata fields for efficient querying and partitioning

/// Order book snapshot data
/// Represents the current state of the order book for an instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookSnapshot {
    pub symbol: String,
    pub venue: String,
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
    pub seq_id: u64,
    pub instrument_class: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ingestion_timestamp: OffsetDateTime,
}

/// Trade execution data
/// Represents an individual trade that occurred on the exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSnapshot {
    pub symbol: String,
    pub venue: String,
    pub trade_id: String,
    pub price: f64,
    pub amount: f64,
    pub side: String, // "buy" or "sell"
    pub seq_id: Option<u64>,
    pub instrument_class: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub ingestion_timestamp: OffsetDateTime,

    // Extended Deribit fields
    pub contracts: Option<f64>,       // Trade size in contract units
    pub index_price: Option<f64>,     // Index Price at the moment of trade
    pub mark_price: Option<f64>,      // Mark Price at the moment of trade
    pub tick_direction: Option<i32>,  // Direction of the "tick" (0-3)
}

/// Ticker data with comprehensive market information (flattened structure)
/// Represents the current market state for an instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerRow {
    pub timestamp: i64,
    pub ingestion_timestamp: i64,
    pub venue: String,
    pub state: u8,
    pub symbol: String,
    pub index_price: Option<f64>,
    pub settlement_price: Option<f64>,
    pub open_interest: Option<f64>,
    pub mark_price: Option<f64>,
    pub best_bid_price: Option<f64>,
    pub mark_iv: Option<f64>,
    pub ask_iv: Option<f64>,
    pub bid_iv: Option<f64>,
    pub underlying_price: Option<f64>,
    pub underlying_index: Option<String>,
    pub best_ask_price: Option<f64>,
    pub interest_rate: Option<f64>,
    pub estimated_delivery_price: Option<f64>,
    pub best_ask_amount: Option<f64>,
    pub best_bid_amount: Option<f64>,
    pub current_funding: Option<f64>,
    pub delivery_price: Option<f64>,
    pub funding_8h: Option<f64>,
    pub interest_value: Option<f64>,
    pub greeks_delta: Option<f64>,
    pub greeks_gamma: Option<f64>,
    pub greeks_vega: Option<f64>,
    pub greeks_theta: Option<f64>,
    pub greeks_rho: Option<f64>,
}

/// Unified market data enum for streaming
/// Tagged with type for easy deserialization and routing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "data_type", rename_all = "lowercase")]
pub enum MarketData {
    Orderbook(OrderBookSnapshot),
    Trade(TradeSnapshot),
    Ticker(TickerRow),
}

#[async_trait]
pub trait Exchange: Send + Sync {
    fn name(&self) -> &str;

    async fn subscribe(&mut self, symbols: &[String]) -> Result<()>;

    async fn unsubscribe(&mut self, symbols: &[String]) -> Result<()>;

    async fn connect_orderbook(&mut self) -> Result<BoxStream<'static, Result<MarketData>>>;

    async fn connect_trades(&mut self) -> Result<BoxStream<'static, Result<MarketData>>>;

    async fn connect_ticker(&mut self) -> Result<BoxStream<'static, Result<MarketData>>>;
}