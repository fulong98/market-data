use serde::{Deserialize, Serialize};
use crate::errors::Result;
use futures::stream::BoxStream;
use async_trait::async_trait;

// Unified Kafka schema for market data
// All data types include common metadata fields for efficient querying and partitioning

/// Order book level with price and amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: f64,
    pub amount: f64,
}

/// Greeks for options (only applicable to options instruments)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Greeks {
    pub delta: f64,
    pub gamma: f64,
    pub rho: f64,
    pub theta: f64,
    pub vega: f64,
}

/// 24-hour statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats24h {
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub volume: Option<f64>,           // Volume in base currency
    pub volume_usd: Option<f64>,       // Volume in USD (futures only)
    pub price_change: Option<f64>,     // 24h price change percentage
}

/// Order book snapshot data
/// Represents the current state of the order book for an instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookData {
    // Common metadata
    pub exchange: String,
    pub instrument_name: String,
    pub timestamp: u64,

    // Order book data
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub change_id: Option<i64>,

    // Best bid/ask summary
    pub best_bid_price: Option<f64>,
    pub best_bid_amount: Option<f64>,
    pub best_ask_price: Option<f64>,
    pub best_ask_amount: Option<f64>,
}

/// Trade execution data
/// Represents an individual trade that occurred on the exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    // Common metadata
    pub exchange: String,
    pub instrument_name: String,
    pub timestamp: u64,

    // Trade specifics
    pub trade_id: String,
    pub trade_seq: Option<u64>,        // Sequence number within instrument
    pub price: f64,
    pub amount: f64,
    pub direction: String,              // "buy" or "sell"

    // Market context
    pub index_price: Option<f64>,
    pub mark_price: Option<f64>,
    pub iv: Option<f64>,                // Implied volatility (options only)
    pub liquidation: Option<String>,    // "M", "T", "MT" for liquidation trades
}

/// Ticker data with comprehensive market information
/// Represents the current market state for an instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickerData {
    // Common metadata
    pub exchange: String,
    pub instrument_name: String,
    pub timestamp: u64,

    // Price data
    pub last_price: Option<f64>,
    pub mark_price: f64,
    pub index_price: f64,
    pub best_bid_price: Option<f64>,
    pub best_ask_price: Option<f64>,
    pub best_bid_amount: f64,
    pub best_ask_amount: f64,

    // Price bounds (for futures)
    pub max_price: Option<f64>,
    pub min_price: Option<f64>,

    // Market state
    pub state: String,                  // "open" or "closed"
    pub open_interest: f64,

    // Funding (perpetuals only)
    pub current_funding: Option<f64>,
    pub funding_8h: Option<f64>,
    pub interest_value: Option<f64>,

    // Settlement (derivatives)
    pub settlement_price: Option<f64>,
    pub delivery_price: Option<f64>,
    pub estimated_delivery_price: Option<f64>,

    // Options-specific
    pub ask_iv: Option<f64>,           // Implied volatility for best ask
    pub bid_iv: Option<f64>,           // Implied volatility for best bid
    pub mark_iv: Option<f64>,          // Implied volatility for mark price
    pub underlying_price: Option<f64>,
    pub underlying_index: Option<String>,
    pub interest_rate: Option<f64>,
    pub greeks: Option<Greeks>,

    // Statistics
    pub stats: Stats24h,
}

/// Unified market data enum for streaming
/// Tagged with type for easy deserialization and routing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "data_type", rename_all = "lowercase")]
pub enum MarketData {
    Orderbook(OrderbookData),
    Trade(TradeData),
    Ticker(TickerData),
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