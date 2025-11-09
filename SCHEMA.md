# Market Data Schema Documentation

This document describes the unified schema for market data stored in Kafka and Redis.

## Overview

The market data system uses a unified schema that supports three types of market data:
1. **Orderbook** - Order book snapshots with bid/ask levels
2. **Trade** - Individual trade executions
3. **Ticker** - Comprehensive market state information

All data is serialized as JSON and includes a `data_type` field for easy routing and deserialization.

## Common Fields

All market data types include these common metadata fields:

| Field | Type | Description |
|-------|------|-------------|
| `exchange` | string | Exchange identifier (e.g., "deribit") |
| `instrument_name` | string | Instrument identifier (e.g., "BTC-PERPETUAL", "ETH-25DEC25-3000-C") |
| `timestamp` | u64 | Unix timestamp in milliseconds |

## Data Types

### 1. Orderbook Data

Represents the current state of the order book for an instrument.

```json
{
  "data_type": "orderbook",
  "exchange": "deribit",
  "instrument_name": "BTC-PERPETUAL",
  "timestamp": 1699564800000,
  "bids": [
    {"price": 43500.0, "amount": 10.5},
    {"price": 43499.5, "amount": 5.2}
  ],
  "asks": [
    {"price": 43501.0, "amount": 8.3},
    {"price": 43501.5, "amount": 12.1}
  ],
  "change_id": 123456789,
  "best_bid_price": 43500.0,
  "best_bid_amount": 10.5,
  "best_ask_price": 43501.0,
  "best_ask_amount": 8.3
}
```

**Fields:**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `bids` | PriceLevel[] | No | Array of bid levels (descending by price) |
| `asks` | PriceLevel[] | No | Array of ask levels (ascending by price) |
| `change_id` | i64 | Yes | Monotonically increasing change identifier |
| `best_bid_price` | f64 | Yes | Highest bid price |
| `best_bid_amount` | f64 | Yes | Amount at best bid |
| `best_ask_price` | f64 | Yes | Lowest ask price |
| `best_ask_amount` | f64 | Yes | Amount at best ask |

**PriceLevel Structure:**
```json
{
  "price": 43500.0,
  "amount": 10.5
}
```

**Kafka Configuration:**
- Topic: `market-data-orderbook`
- Key: `{exchange}.{instrument_name}` (e.g., "deribit.BTC-PERPETUAL")
- Partitioning: By key to ensure order for each instrument

**Redis Configuration:**
- Key pattern: `{exchange}:{instrument_name}:orderbook`
- TTL: 3 seconds (high-frequency updates)

### 2. Trade Data

Represents an individual trade execution on the exchange.

```json
{
  "data_type": "trade",
  "exchange": "deribit",
  "instrument_name": "BTC-PERPETUAL",
  "timestamp": 1699564800000,
  "trade_id": "123456789",
  "trade_seq": 987654321,
  "price": 43500.5,
  "amount": 1.5,
  "direction": "buy",
  "index_price": 43498.2,
  "mark_price": null,
  "iv": null,
  "liquidation": null
}
```

**Fields:**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `trade_id` | string | No | Unique trade identifier |
| `trade_seq` | u64 | Yes | Sequence number within instrument |
| `price` | f64 | No | Execution price |
| `amount` | f64 | No | Trade amount (in base currency) |
| `direction` | string | No | "buy" or "sell" from taker perspective |
| `index_price` | f64 | Yes | Index price at time of trade |
| `mark_price` | f64 | Yes | Mark price at time of trade |
| `iv` | f64 | Yes | Implied volatility (options only) |
| `liquidation` | string | Yes | "M" (maker), "T" (taker), "MT" (both) for liquidations |

**Kafka Configuration:**
- Topic: `market-data-trades`
- Key: `{exchange}.{instrument_name}`
- Partitioning: By key to maintain order

**Redis Configuration:**
- Key pattern: `{exchange}:{instrument_name}:last_trade`
- TTL: 60 seconds

### 3. Ticker Data

Comprehensive market state information for an instrument. This is the most detailed data type, containing pricing, funding, settlement, options Greeks, and 24h statistics.

```json
{
  "data_type": "ticker",
  "exchange": "deribit",
  "instrument_name": "BTC-PERPETUAL",
  "timestamp": 1699564800000,
  "last_price": 43500.0,
  "mark_price": 43499.5,
  "index_price": 43498.2,
  "best_bid_price": 43500.0,
  "best_ask_price": 43501.0,
  "best_bid_amount": 10.5,
  "best_ask_amount": 8.3,
  "max_price": 44000.0,
  "min_price": 43000.0,
  "state": "open",
  "open_interest": 125000.0,
  "current_funding": 0.0001,
  "funding_8h": 0.0003,
  "interest_value": null,
  "settlement_price": null,
  "delivery_price": null,
  "estimated_delivery_price": 43500.0,
  "ask_iv": null,
  "bid_iv": null,
  "mark_iv": null,
  "underlying_price": null,
  "underlying_index": null,
  "interest_rate": null,
  "greeks": null,
  "stats": {
    "high": 44200.0,
    "low": 42800.0,
    "volume": 15234.5,
    "volume_usd": 662000000.0,
    "price_change": 1.25
  }
}
```

**Core Price Fields:**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `last_price` | f64 | Yes | Last traded price |
| `mark_price` | f64 | No | Fair value mark price |
| `index_price` | f64 | No | Underlying index price |
| `best_bid_price` | f64 | Yes | Current best bid |
| `best_ask_price` | f64 | Yes | Current best ask |
| `best_bid_amount` | f64 | No | Amount at best bid |
| `best_ask_amount` | f64 | No | Amount at best ask |

**Price Bounds (Futures):**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `max_price` | f64 | Yes | Maximum allowed order price |
| `min_price` | f64 | Yes | Minimum allowed order price |

**Market State:**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `state` | string | No | "open" or "closed" |
| `open_interest` | f64 | No | Outstanding contracts (USD for perpetuals/inverse futures, base currency for linear futures/options) |

**Funding (Perpetuals Only):**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `current_funding` | f64 | Yes | Current funding rate |
| `funding_8h` | f64 | Yes | 8-hour funding rate |
| `interest_value` | f64 | Yes | Value for realized_funding calculation |

**Settlement (Derivatives):**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `settlement_price` | f64 | Yes | Settlement price (when state=open) |
| `delivery_price` | f64 | Yes | Delivery price (when state=closed) |
| `estimated_delivery_price` | f64 | Yes | Estimated delivery/expiration price |

**Options-Specific Fields:**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `ask_iv` | f64 | Yes | Implied volatility for best ask |
| `bid_iv` | f64 | Yes | Implied volatility for best bid |
| `mark_iv` | f64 | Yes | Implied volatility for mark price |
| `underlying_price` | f64 | Yes | Underlying asset price |
| `underlying_index` | string | Yes | Underlying index or "index_price" |
| `interest_rate` | f64 | Yes | Interest rate for IV calculations |
| `greeks` | Greeks | Yes | Option Greeks (see below) |

**Greeks Structure (Options Only):**
```json
{
  "delta": 0.5234,
  "gamma": 0.0002,
  "rho": 0.1234,
  "theta": -12.34,
  "vega": 42.56
}
```

**24-Hour Statistics:**

| Field | Type | Nullable | Description |
|-------|------|----------|-------------|
| `stats.high` | f64 | Yes | Highest price in 24h |
| `stats.low` | f64 | Yes | Lowest price in 24h |
| `stats.volume` | f64 | Yes | Volume in base currency |
| `stats.volume_usd` | f64 | Yes | Volume in USD (futures only) |
| `stats.price_change` | f64 | Yes | 24h price change (percentage) |

**Kafka Configuration:**
- Topic: `market-data-ticker`
- Key: `{exchange}.{instrument_name}`
- Partitioning: By key

**Redis Configuration:**
- Key pattern: `{exchange}:{instrument_name}:ticker`
- TTL: 300 seconds (5 minutes)

## Usage Examples

### Consuming from Kafka

```rust
use rdkafka::consumer::{Consumer, StreamConsumer};
use market_data::exchanges::deribit::models::MarketData;

let consumer: StreamConsumer = /* ... */;

while let Ok(message) = consumer.recv().await {
    if let Some(payload) = message.payload_view::<str>() {
        let data: MarketData = serde_json::from_str(payload?)?;

        match data {
            MarketData::Orderbook(ob) => {
                println!("Orderbook for {}: {} levels", ob.instrument_name, ob.bids.len());
            }
            MarketData::Trade(trade) => {
                println!("Trade: {} @ {} {}", trade.amount, trade.price, trade.direction);
            }
            MarketData::Ticker(ticker) => {
                println!("Ticker: mark={}, index={}", ticker.mark_price, ticker.index_price);
            }
        }
    }
}
```

### Querying from Redis

```bash
# Get latest orderbook
redis-cli GET "deribit:BTC-PERPETUAL:orderbook"

# Get last trade
redis-cli GET "deribit:BTC-PERPETUAL:last_trade"

# Get ticker
redis-cli GET "deribit:BTC-PERPETUAL:ticker"

# Pattern matching to find all instruments
redis-cli KEYS "deribit:*:ticker"
```

## Data Freshness & TTL

| Data Type | Update Frequency | Redis TTL | Reasoning |
|-----------|-----------------|-----------|-----------|
| Orderbook | 100ms | 3 sec | High-frequency, large payload |
| Trade | Per trade | 60 sec | Event-driven, smaller payload |
| Ticker | 100ms | 300 sec | Moderate frequency, comprehensive data |

## Schema Evolution

The schema is designed to be forward-compatible:
- New optional fields can be added without breaking existing consumers
- The `data_type` tag enables routing to appropriate handlers
- Nullable fields allow for exchange-specific data without schema fragmentation

## Partitioning Strategy

Kafka topics are partitioned by `{exchange}.{instrument_name}` to:
- Ensure ordering of updates for each instrument
- Enable parallel processing across different instruments
- Facilitate instrument-level consumption patterns
- Support horizontal scaling

## Best Practices

1. **Always check nullable fields** before using them (e.g., `greeks` for non-options)
2. **Use the `data_type` tag** for deserialization routing
3. **Monitor Redis TTL** - data may not be available if TTL expires
4. **Handle deserialization errors** gracefully for schema evolution
5. **Use instrument_name as primary key** for all queries and storage
6. **Leverage Kafka consumer groups** for parallel processing
7. **Consider ClickHouse** for historical data storage and analytics
