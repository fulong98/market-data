use super::models::{
    Exchange, MarketData, OrderBookSnapshot, TradeSnapshot, TickerRow,
};
use crate::errors::{MarketDataError, Result};
use async_trait::async_trait;
use deribit::{
    models::{
        subscription::{
            GroupedBookData, PublicSubscribeRequest, PublicUnsubscribeRequest, SubscriptionData,
            SubscriptionMessage, SubscriptionParams, TickerData as DeribitTickerData,
            TradesData as DeribitTradesData, HeartbeatType,
        },
        session_management::SetHeartbeatRequest,
        TestRequest,
    },
    DeribitAPIClient, DeribitSubscriptionClient,
};
use futures::{stream::BoxStream, StreamExt};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, info};

pub struct DeribitConfig {
    pub testnet: bool,
    pub heartbeat_interval: u64,
}

impl Default for DeribitConfig {
    fn default() -> Self {
        Self {
            testnet: false,
            heartbeat_interval: 10,
        }
    }
}

pub struct Deribit {
    api_client: Arc<RwLock<DeribitAPIClient>>,
    subscription_client: Arc<RwLock<DeribitSubscriptionClient>>,
    subscribed_symbols: Arc<RwLock<HashSet<String>>>,
    symbol_receiver: Arc<RwLock<Option<mpsc::Receiver<Vec<String>>>>>,
}

impl Deribit {
    pub async fn new(
        config: DeribitConfig,
        symbol_rx: mpsc::Receiver<Vec<String>>,
    ) -> Result<Self> {
        info!("Connecting to Deribit WebSocket...");

        // Build the Deribit client
        let drb = if config.testnet {
            deribit::Deribit::builder()
                .testnet(true)
                .build()
                .map_err(|e| MarketDataError::ConnectionError(e.to_string()))?
        } else {
            deribit::Deribit::new()
        };

        let (mut api_client, subscription_client) = drb
            .connect()
            .await
            .map_err(|e| MarketDataError::WebSocketError(e.to_string()))?;

        // Set heartbeat
        info!("Setting heartbeat interval to {}s", config.heartbeat_interval);
        let _ = api_client
            .call(SetHeartbeatRequest::with_interval(config.heartbeat_interval))
            .await
            .map_err(|e| MarketDataError::ConnectionError(e.to_string()))?
            .await;

        Ok(Self {
            api_client: Arc::new(RwLock::new(api_client)),
            subscription_client: Arc::new(RwLock::new(subscription_client)),
            subscribed_symbols: Arc::new(RwLock::new(HashSet::new())),
            symbol_receiver: Arc::new(RwLock::new(Some(symbol_rx))),
        })
    }

    async fn subscribe_channels(&self, channels: Vec<String>) -> Result<()> {
        let mut api_client = self.api_client.write().await;

        debug!("Subscribing to channels: {:?}", channels);
        let req = PublicSubscribeRequest::new(&channels);

        api_client
            .call(req)
            .await
            .map_err(|e| MarketDataError::WebSocketError(e.to_string()))?
            .await
            .map_err(|e| MarketDataError::WebSocketError(e.to_string()))?;

        Ok(())
    }

    async fn unsubscribe_channels(&self, channels: Vec<String>) -> Result<()> {
        let mut api_client = self.api_client.write().await;

        debug!("Unsubscribing from channels: {:?}", channels);
        let req = PublicUnsubscribeRequest::new(&channels);

        api_client
            .call(req)
            .await
            .map_err(|e| MarketDataError::WebSocketError(e.to_string()))?
            .await
            .map_err(|e| MarketDataError::WebSocketError(e.to_string()))?;

        Ok(())
    }

    fn convert_grouped_book_to_market_data(data: GroupedBookData) -> MarketData {
        use time::OffsetDateTime;

        let timestamp = OffsetDateTime::from_unix_timestamp_nanos((data.timestamp as i128) * 1_000_000)
            .unwrap_or_else(|_| OffsetDateTime::now_utc());
        let ingestion_timestamp = OffsetDateTime::now_utc();

        MarketData::Orderbook(OrderBookSnapshot {
            symbol: data.instrument_name.clone(),
            venue: "deribit".to_string(),
            bids: data.bids,
            asks: data.asks,
            seq_id: data.change_id as u64,
            instrument_class: None, // Not provided in Deribit book data
            timestamp,
            ingestion_timestamp,
        })
    }

    fn convert_trades_to_market_data(trades: Vec<DeribitTradesData>) -> Vec<MarketData> {
        use time::OffsetDateTime;

        trades
            .into_iter()
            .map(|trade| {
                let timestamp = OffsetDateTime::from_unix_timestamp_nanos((trade.timestamp as i128) * 1_000_000)
                    .unwrap_or_else(|_| OffsetDateTime::now_utc());
                let ingestion_timestamp = OffsetDateTime::now_utc();

                MarketData::Trade(TradeSnapshot {
                    symbol: trade.instrument_name.clone(),
                    venue: "deribit".to_string(),
                    trade_id: trade.trade_id,
                    price: trade.price,
                    amount: trade.amount,
                    side: format!("{:?}", trade.direction).to_lowercase(),
                    seq_id: Some(trade.trade_seq),
                    instrument_class: None,
                    timestamp,
                    ingestion_timestamp,
                    contracts: Some(trade.amount), // Use amount as contracts
                    index_price: Some(trade.index_price),
                    mark_price: None, // Not provided in trades subscription
                    tick_direction: Some(trade.tick_direction as i32),
                })
            })
            .collect()
    }

    fn convert_ticker_to_market_data(ticker: DeribitTickerData) -> MarketData {
        use time::OffsetDateTime;

        // Convert state to u8: open=1, closed=0
        let state = match format!("{:?}", ticker.state).to_lowercase().as_str() {
            "open" => 1,
            _ => 0,
        };

        let ingestion_timestamp = OffsetDateTime::now_utc().unix_timestamp();

        MarketData::Ticker(TickerRow {
            timestamp: ticker.timestamp as i64,
            ingestion_timestamp,
            venue: "deribit".to_string(),
            state,
            symbol: ticker.instrument_name.clone(),
            index_price: Some(ticker.index_price),
            settlement_price: ticker.settlement_price,
            open_interest: Some(ticker.open_interest),
            mark_price: Some(ticker.mark_price),
            best_bid_price: ticker.best_bid_price,
            mark_iv: ticker.mark_iv,
            ask_iv: ticker.ask_iv,
            bid_iv: ticker.bid_iv,
            underlying_price: ticker.underlying_price,
            underlying_index: ticker.underlying_index.clone(),
            best_ask_price: ticker.best_ask_price,
            interest_rate: ticker.interest_rate,
            estimated_delivery_price: Some(ticker.estimated_delivery_price),
            best_ask_amount: Some(ticker.best_ask_amount),
            best_bid_amount: Some(ticker.best_bid_amount),
            current_funding: ticker.current_funding,
            delivery_price: ticker.delivery_price,
            funding_8h: ticker.funding_8h,
            interest_value: None, // Not available in subscription data
            greeks_delta: ticker.greeks.as_ref().map(|g| g.delta),
            greeks_gamma: ticker.greeks.as_ref().map(|g| g.gamma),
            greeks_vega: ticker.greeks.as_ref().map(|g| g.vega),
            greeks_theta: ticker.greeks.as_ref().map(|g| g.theta),
            greeks_rho: ticker.greeks.as_ref().map(|g| g.rho),
        })
    }

    async fn start_dynamic_subscription_handler(&self) {
        let symbols = self.subscribed_symbols.clone();
        let _api_client = self.api_client.clone();
        let symbol_receiver = self.symbol_receiver.clone();

        tokio::spawn(async move {
            let rx = {
                let mut guard = symbol_receiver.write().await;
                guard.take()
            };

            if let Some(mut receiver) = rx {
                while let Some(new_symbols) = receiver.recv().await {
                    info!("Received dynamic symbol update: {:?}", new_symbols);

                    // This is where you'd implement logic to subscribe to new symbols
                    // For now, just update the symbols list
                    let mut symbols_guard = symbols.write().await;
                    for symbol in new_symbols {
                        symbols_guard.insert(symbol);
                    }
                }
            }
        });
    }
}

#[async_trait]
impl Exchange for Deribit {
    fn name(&self) -> &str {
        "deribit"
    }

    async fn subscribe(&mut self, symbols: &[String]) -> Result<()> {
        // Store symbols for later use
        let mut subscribed = self.subscribed_symbols.write().await;
        for symbol in symbols {
            subscribed.insert(symbol.clone());
        }
        Ok(())
    }

    async fn unsubscribe(&mut self, symbols: &[String]) -> Result<()> {
        let mut subscribed = self.subscribed_symbols.write().await;
        for symbol in symbols {
            subscribed.remove(symbol);
        }
        Ok(())
    }

    async fn connect_orderbook(&mut self) -> Result<BoxStream<'static, Result<MarketData>>> {
        let symbols = self.subscribed_symbols.read().await.clone();

        // Build channel strings: book.{instrument}.none.10.100ms (grouped book with 10 levels, 100ms interval)
        let channels: Vec<String> = symbols
            .iter()
            .map(|symbol| format!("book.{}.none.10.100ms", symbol))
            .collect();

        if channels.is_empty() {
            return Err(MarketDataError::ConfigError(
                "No symbols subscribed for orderbook".to_string(),
            ));
        }

        info!("Subscribing to orderbook channels: {:?}", channels);
        self.subscribe_channels(channels).await?;

        // Start dynamic subscription handler
        self.start_dynamic_subscription_handler().await;

        let subscription_client = self.subscription_client.clone();
        let api_client = self.api_client.clone();

        let stream = async_stream::stream! {
            let mut sub_client = subscription_client.write().await;

            while let Some(message_result) = sub_client.next().await {
                match message_result {
                    Ok(SubscriptionMessage {
                        params: SubscriptionParams::Heartbeat { r#type: HeartbeatType::TestRequest },
                        ..
                    }) => {
                        // Respond to heartbeat test request
                        debug!("Received heartbeat test request, responding...");
                        let mut api = api_client.write().await;
                        if let Err(e) = api.call(TestRequest::default()).await {
                            debug!("Failed to respond to heartbeat: {:?}", e);
                        }
                    }
                    Ok(SubscriptionMessage {
                        params: SubscriptionParams::Subscription(SubscriptionData::GroupedBook(data)),
                        ..
                    }) => {
                        let market_data = Self::convert_grouped_book_to_market_data(data.data);
                        yield Ok(market_data);
                    }
                    Err(e) => {
                        yield Err(MarketDataError::WebSocketError(e.to_string()));
                    }
                    _ => {
                        // Ignore other message types (heartbeats, other subscriptions)
                        debug!("Ignoring non-orderbook message");
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    async fn connect_trades(&mut self) -> Result<BoxStream<'static, Result<MarketData>>> {
        let symbols = self.subscribed_symbols.read().await.clone();

        // Build channel strings: trades.{instrument}.100ms
        let channels: Vec<String> = symbols
            .iter()
            .map(|symbol| format!("trades.{}.100ms", symbol))
            .collect();

        if channels.is_empty() {
            return Err(MarketDataError::ConfigError(
                "No symbols subscribed for trades".to_string(),
            ));
        }

        info!("Subscribing to trade channels: {:?}", channels);
        self.subscribe_channels(channels).await?;

        let subscription_client = self.subscription_client.clone();
        let api_client = self.api_client.clone();

        let stream = async_stream::stream! {
            let mut sub_client = subscription_client.write().await;

            while let Some(message_result) = sub_client.next().await {
                match message_result {
                    Ok(SubscriptionMessage {
                        params: SubscriptionParams::Heartbeat { r#type: HeartbeatType::TestRequest },
                        ..
                    }) => {
                        // Respond to heartbeat test request
                        debug!("Received heartbeat test request, responding...");
                        let mut api = api_client.write().await;
                        if let Err(e) = api.call(TestRequest::default()).await {
                            debug!("Failed to respond to heartbeat: {:?}", e);
                        }
                    }
                    Ok(SubscriptionMessage {
                        params: SubscriptionParams::Subscription(SubscriptionData::Trades(data)),
                        ..
                    }) => {
                        for market_data in Self::convert_trades_to_market_data(data.data) {
                            yield Ok(market_data);
                        }
                    }
                    Err(e) => {
                        yield Err(MarketDataError::WebSocketError(e.to_string()));
                    }
                    _ => {
                        // Ignore other message types
                        debug!("Ignoring non-trade message");
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }

    async fn connect_ticker(&mut self) -> Result<BoxStream<'static, Result<MarketData>>> {
        let symbols = self.subscribed_symbols.read().await.clone();

        // Build channel strings: ticker.{instrument}.100ms
        let channels: Vec<String> = symbols
            .iter()
            .map(|symbol| format!("ticker.{}.100ms", symbol))
            .collect();

        if channels.is_empty() {
            return Err(MarketDataError::ConfigError(
                "No symbols subscribed for ticker".to_string(),
            ));
        }

        info!("Subscribing to ticker channels: {:?}", channels);
        self.subscribe_channels(channels).await?;

        let subscription_client = self.subscription_client.clone();
        let api_client = self.api_client.clone();

        let stream = async_stream::stream! {
            let mut sub_client = subscription_client.write().await;

            while let Some(message_result) = sub_client.next().await {
                match message_result {
                    Ok(SubscriptionMessage {
                        params: SubscriptionParams::Heartbeat { r#type: HeartbeatType::TestRequest },
                        ..
                    }) => {
                        // Respond to heartbeat test request
                        debug!("Received heartbeat test request, responding...");
                        let mut api = api_client.write().await;
                        if let Err(e) = api.call(TestRequest::default()).await {
                            debug!("Failed to respond to heartbeat: {:?}", e);
                        }
                    }
                    Ok(SubscriptionMessage {
                        params: SubscriptionParams::Subscription(SubscriptionData::Ticker(data)),
                        ..
                    }) => {
                        let market_data = Self::convert_ticker_to_market_data(data.data);
                        yield Ok(market_data);
                    }
                    Err(e) => {
                        yield Err(MarketDataError::WebSocketError(e.to_string()));
                    }
                    _ => {
                        // Ignore other message types
                        debug!("Ignoring non-ticker message");
                    }
                }
            }
        };

        Ok(Box::pin(stream))
    }
}
