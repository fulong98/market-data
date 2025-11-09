use super::models::{
    Exchange, Greeks, MarketData, OrderbookData, PriceLevel, Stats24h, TickerData, TradeData,
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

    fn convert_grouped_book_to_market_data(&self, data: GroupedBookData) -> MarketData {
        // Calculate best bid/ask from the book data
        let best_bid_price = data.bids.first().map(|(price, _)| *price);
        let best_bid_amount = data.bids.first().map(|(_, amount)| *amount);
        let best_ask_price = data.asks.first().map(|(price, _)| *price);
        let best_ask_amount = data.asks.first().map(|(_, amount)| *amount);

        MarketData::Orderbook(OrderbookData {
            exchange: "deribit".to_string(),
            instrument_name: data.instrument_name.clone(),
            timestamp: data.timestamp,
            bids: data
                .bids
                .into_iter()
                .map(|(price, amount)| PriceLevel { price, amount })
                .collect(),
            asks: data
                .asks
                .into_iter()
                .map(|(price, amount)| PriceLevel { price, amount })
                .collect(),
            change_id: Some(data.change_id),
            best_bid_price,
            best_bid_amount,
            best_ask_price,
            best_ask_amount,
        })
    }

    fn convert_trades_to_market_data(&self, trades: Vec<DeribitTradesData>) -> Vec<MarketData> {
        trades
            .into_iter()
            .map(|trade| {
                MarketData::Trade(TradeData {
                    exchange: "deribit".to_string(),
                    instrument_name: trade.instrument_name.clone(),
                    timestamp: trade.timestamp,
                    trade_id: trade.trade_id,
                    trade_seq: Some(trade.trade_seq),
                    price: trade.price,
                    amount: trade.amount,
                    direction: format!("{:?}", trade.direction).to_lowercase(),
                    index_price: Some(trade.index_price),
                    mark_price: None, // Not provided in trades subscription
                    iv: trade.iv,
                    liquidation: trade.liquidation.map(|l| format!("{:?}", l)),
                })
            })
            .collect()
    }

    fn convert_ticker_to_market_data(&self, ticker: DeribitTickerData) -> MarketData {
        MarketData::Ticker(TickerData {
            exchange: "deribit".to_string(),
            instrument_name: ticker.instrument_name.clone(),
            timestamp: ticker.timestamp,
            last_price: ticker.last_price,
            mark_price: ticker.mark_price,
            index_price: ticker.index_price,
            best_bid_price: ticker.best_bid_price,
            best_ask_price: ticker.best_ask_price,
            best_bid_amount: ticker.best_bid_amount,
            best_ask_amount: ticker.best_ask_amount,
            max_price: Some(ticker.max_price),
            min_price: Some(ticker.min_price),
            state: format!("{:?}", ticker.state).to_lowercase(),
            open_interest: ticker.open_interest,
            current_funding: ticker.current_funding,
            funding_8h: ticker.funding_8h,
            interest_value: None, // Not available in subscription data
            settlement_price: ticker.settlement_price,
            delivery_price: ticker.delivery_price,
            estimated_delivery_price: Some(ticker.estimated_delivery_price),
            ask_iv: ticker.ask_iv,
            bid_iv: ticker.bid_iv,
            mark_iv: ticker.mark_iv,
            underlying_price: ticker.underlying_price,
            underlying_index: ticker.underlying_index.clone(),
            interest_rate: ticker.interest_rate,
            greeks: ticker.greeks.map(|g| Greeks {
                delta: g.delta,
                gamma: g.gamma,
                rho: g.rho,
                theta: g.theta,
                vega: g.vega,
            }),
            stats: Stats24h {
                high: ticker.stats.high,
                low: ticker.stats.low,
                volume: ticker.stats.volume,
                volume_usd: ticker.stats.volume_usd,
                price_change: ticker.stats.price_change,
            },
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
                        // Calculate best bid/ask from the book data
                        let best_bid_price = data.data.bids.first().map(|(price, _)| *price);
                        let best_bid_amount = data.data.bids.first().map(|(_, amount)| *amount);
                        let best_ask_price = data.data.asks.first().map(|(price, _)| *price);
                        let best_ask_amount = data.data.asks.first().map(|(_, amount)| *amount);

                        let market_data = MarketData::Orderbook(OrderbookData {
                            exchange: "deribit".to_string(),
                            instrument_name: data.data.instrument_name.clone(),
                            timestamp: data.data.timestamp,
                            bids: data.data.bids
                                .into_iter()
                                .map(|(price, amount)| PriceLevel { price, amount })
                                .collect(),
                            asks: data.data.asks
                                .into_iter()
                                .map(|(price, amount)| PriceLevel { price, amount })
                                .collect(),
                            change_id: Some(data.data.change_id),
                            best_bid_price,
                            best_bid_amount,
                            best_ask_price,
                            best_ask_amount,
                        });
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
                        for trade in data.data {
                            let market_data = MarketData::Trade(TradeData {
                                exchange: "deribit".to_string(),
                                instrument_name: trade.instrument_name.clone(),
                                timestamp: trade.timestamp,
                                trade_id: trade.trade_id,
                                trade_seq: Some(trade.trade_seq),
                                price: trade.price,
                                amount: trade.amount,
                                direction: format!("{:?}", trade.direction).to_lowercase(),
                                index_price: Some(trade.index_price),
                                mark_price: None,
                                iv: trade.iv,
                                liquidation: trade.liquidation.map(|l| format!("{:?}", l)),
                            });
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
                        let market_data = MarketData::Ticker(TickerData {
                            exchange: "deribit".to_string(),
                            instrument_name: data.data.instrument_name.clone(),
                            timestamp: data.data.timestamp,
                            last_price: data.data.last_price,
                            mark_price: data.data.mark_price,
                            index_price: data.data.index_price,
                            best_bid_price: data.data.best_bid_price,
                            best_ask_price: data.data.best_ask_price,
                            best_bid_amount: data.data.best_bid_amount,
                            best_ask_amount: data.data.best_ask_amount,
                            max_price: Some(data.data.max_price),
                            min_price: Some(data.data.min_price),
                            state: format!("{:?}", data.data.state).to_lowercase(),
                            open_interest: data.data.open_interest,
                            current_funding: data.data.current_funding,
                            funding_8h: data.data.funding_8h,
                            interest_value: None,
                            settlement_price: data.data.settlement_price,
                            delivery_price: data.data.delivery_price,
                            estimated_delivery_price: Some(data.data.estimated_delivery_price),
                            ask_iv: data.data.ask_iv,
                            bid_iv: data.data.bid_iv,
                            mark_iv: data.data.mark_iv,
                            underlying_price: data.data.underlying_price,
                            underlying_index: data.data.underlying_index.clone(),
                            interest_rate: data.data.interest_rate,
                            greeks: data.data.greeks.map(|g| Greeks {
                                delta: g.delta,
                                gamma: g.gamma,
                                rho: g.rho,
                                theta: g.theta,
                                vega: g.vega,
                            }),
                            stats: Stats24h {
                                high: data.data.stats.high,
                                low: data.data.stats.low,
                                volume: data.data.stats.volume,
                                volume_usd: data.data.stats.volume_usd,
                                price_change: data.data.stats.price_change,
                            },
                        });
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
