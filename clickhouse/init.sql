-- Market Data ClickHouse Schema
-- This schema is optimized for time-series market data storage and analysis

-- Create database if not exists
CREATE DATABASE IF NOT EXISTS market_data;

USE market_data;

-- Orderbook Table
-- Stores order book snapshots with bid/ask levels
CREATE TABLE IF NOT EXISTS orderbook (
    timestamp DateTime64(3) CODEC(Delta, ZSTD),
    ingestion_timestamp DateTime64(3) CODEC(Delta, ZSTD),
    venue LowCardinality(String),
    symbol String,
    seq_id UInt64,
    instrument_class Nullable(String),
    bids Nested(
        price Float64,
        amount Float64
    ),
    asks Nested(
        price Float64,
        amount Float64
    ),
    date Date DEFAULT toDate(timestamp)
) ENGINE = MergeTree()
PARTITION BY (venue, toYYYYMM(timestamp))
ORDER BY (venue, symbol, timestamp)
TTL timestamp + INTERVAL 30 DAY
SETTINGS index_granularity = 8192;

-- Trades Table
-- Stores individual trade executions
CREATE TABLE IF NOT EXISTS trades (
    timestamp DateTime64(3) CODEC(Delta, ZSTD),
    ingestion_timestamp DateTime64(3) CODEC(Delta, ZSTD),
    venue LowCardinality(String),
    symbol String,
    trade_id String,
    seq_id Nullable(UInt64),
    price Float64 CODEC(Gorilla, ZSTD),
    amount Float64 CODEC(Gorilla, ZSTD),
    side LowCardinality(String),
    instrument_class Nullable(String),
    contracts Nullable(Float64),
    index_price Nullable(Float64),
    mark_price Nullable(Float64),
    tick_direction Nullable(Int32),
    date Date DEFAULT toDate(timestamp)
) ENGINE = MergeTree()
PARTITION BY (venue, toYYYYMM(timestamp))
ORDER BY (venue, symbol, timestamp)
TTL timestamp + INTERVAL 90 DAY
SETTINGS index_granularity = 8192;

-- Ticker Table
-- Stores comprehensive market state information
CREATE TABLE IF NOT EXISTS ticker (
    timestamp Int64 CODEC(Delta, ZSTD),
    ingestion_timestamp Int64 CODEC(Delta, ZSTD),
    venue LowCardinality(String),
    state UInt8,
    symbol String,
    index_price Nullable(Float64) CODEC(Gorilla, ZSTD),
    settlement_price Nullable(Float64),
    open_interest Nullable(Float64),
    mark_price Nullable(Float64) CODEC(Gorilla, ZSTD),
    best_bid_price Nullable(Float64),
    mark_iv Nullable(Float64),
    ask_iv Nullable(Float64),
    bid_iv Nullable(Float64),
    underlying_price Nullable(Float64),
    underlying_index Nullable(String),
    best_ask_price Nullable(Float64),
    interest_rate Nullable(Float64),
    estimated_delivery_price Nullable(Float64),
    best_ask_amount Nullable(Float64),
    best_bid_amount Nullable(Float64),
    current_funding Nullable(Float64),
    delivery_price Nullable(Float64),
    funding_8h Nullable(Float64),
    interest_value Nullable(Float64),
    greeks_delta Nullable(Float64),
    greeks_gamma Nullable(Float64),
    greeks_vega Nullable(Float64),
    greeks_theta Nullable(Float64),
    greeks_rho Nullable(Float64),
    date Date DEFAULT toDate(toDateTime(timestamp))
) ENGINE = MergeTree()
PARTITION BY (venue, toYYYYMM(toDateTime(timestamp)))
ORDER BY (venue, symbol, timestamp)
TTL toDateTime(timestamp) + INTERVAL 90 DAY
SETTINGS index_granularity = 8192;

-- Materialized Views for Common Queries

-- Latest ticker per instrument (for fast lookups)
CREATE MATERIALIZED VIEW IF NOT EXISTS ticker_latest
ENGINE = ReplacingMergeTree(timestamp)
PARTITION BY venue
ORDER BY (venue, symbol)
AS SELECT
    venue,
    symbol,
    timestamp,
    mark_price,
    index_price,
    best_bid_price,
    best_ask_price,
    open_interest,
    current_funding,
    funding_8h
FROM ticker;

-- 1-minute OHLCV aggregation from trades
CREATE MATERIALIZED VIEW IF NOT EXISTS trades_ohlcv_1m
ENGINE = SummingMergeTree()
PARTITION BY (venue, toYYYYMM(timestamp_1m))
ORDER BY (venue, symbol, timestamp_1m)
AS SELECT
    venue,
    symbol,
    toStartOfMinute(timestamp) as timestamp_1m,
    argMin(price, timestamp) as open,
    max(price) as high,
    min(price) as low,
    argMax(price, timestamp) as close,
    sum(amount) as volume,
    count() as trade_count
FROM trades
GROUP BY venue, symbol, timestamp_1m;

-- Note: orderbook_spread view removed as OrderBookSnapshot doesn't include best_bid/ask_price fields

-- Create indexes for faster queries
-- Indexes on frequently filtered columns
ALTER TABLE trades ADD INDEX idx_trade_id trade_id TYPE bloom_filter GRANULARITY 4;
ALTER TABLE trades ADD INDEX idx_side side TYPE set(2) GRANULARITY 4;
