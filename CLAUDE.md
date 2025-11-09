# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a cryptocurrency market data collection system written in Rust that ingests real-time market data from exchanges (currently Deribit) and streams it to Kafka topics while maintaining the latest data in Redis. The system is designed for high-throughput, real-time market data processing with observability via Grafana/Loki.

The project uses a workspace structure with two main components:
- **deribit-rs**: A standalone library for Deribit API v2 (WebSocket-based async client)
- **market-data**: The main application with collector binaries for orderbook, trades, and ticker data

## Building and Running

```bash
# Build the entire workspace
cargo build

# Build in release mode
cargo build --release

# Run specific collector binaries
cargo run --bin orderbook_collector
cargo run --bin trades_collector
cargo run --bin ticker_collector

# Run tests
cargo test

# Run tests for deribit-rs library only
cd deribit-rs && cargo test -- --test-threads=1 --nocapture

# Format code
cargo fmt
```

**Important**:
- The project is currently in early development - many components have incomplete implementations
- Collector binaries require environment configuration and running infrastructure (see below)
- Tests for deribit-rs require `DERIBIT_KEY` and `DERIBIT_SECRET` environment variables

## Infrastructure Setup

The project uses Docker Compose for local development infrastructure:

```bash
# Start all infrastructure services
docker-compose up -d

# Stop all services
docker-compose down

# View logs
docker-compose logs -f [service_name]

# Run setup script to initialize Kafka topics and ClickHouse database
./scripts/setup.sh
```

**Services provided by docker-compose.yml**:
- **Kafka** (localhost:29092) - Message broker for market data streams
- **Zookeeper** (localhost:2181) - Required for Kafka coordination
- **ClickHouse** (HTTP: localhost:8123, Native: localhost:9000) - Time-series database for market data storage
- **Redis** (localhost:6379) - Cache for latest market data snapshots
- **Loki** (localhost:3100) - Log aggregation
- **Grafana** (localhost:3000) - Observability dashboard (admin/admin)

## Configuration

Configuration is managed via TOML files in the `config/` directory:

- **config/default.toml** - Main configuration file with settings for:
  - Kafka topics (orderbook, trades, ticker)
  - Redis connection URL
  - ClickHouse connection parameters
  - Exchange configurations
  - Logging settings (level, format)
  - Health check endpoint

The application loads configuration using the `config` crate, allowing environment-specific overrides.

## Architecture

### High-Level Data Flow

1. **Collector Binaries** (`src/bin/*_collector.rs`) - Three separate binaries for collecting different data types:
   - `orderbook_collector` - Order book snapshots and updates
   - `trades_collector` - Executed trades
   - `ticker_collector` - Ticker/price updates

2. **Exchange Abstraction** (`src/exchanges/`):
   - `Exchange` trait defines the interface for exchange connectors
   - Returns async streams of `MarketData` using `BoxStream<'static, Result<MarketData>>`
   - Currently only Deribit is implemented (incomplete)

3. **Data Pipeline**:
   - Exchange connector subscribes to WebSocket channels
   - Market data is parsed into unified `MarketData` enum
   - Data flows to two destinations concurrently:
     - **Kafka**: Persistent streaming via `KafkaProducer` (`infra/kafka_producer.rs`)
     - **Redis**: Latest snapshot via `RedisStorage` (`infra/redis.rs`)

4. **Symbol Management**:
   - Uses mpsc channels to dynamically add/remove trading symbols
   - `KafkaConsumer` reads symbol changes from Kafka topics
   - Collectors spawn tasks per exchange with symbol receivers

### Key Design Patterns

**Async Streams**: Exchange connectors return `BoxStream<'static, Result<MarketData>>` for flexible, composable async data processing.

**Error Handling**: Central `MarketDataError` enum in `src/errors.rs` using `thiserror` crate for all domain errors (WebSocket, Kafka, Redis, ClickHouse, JSON, HTTP, Config).

**Multi-Exchange Support**: Factory pattern in `ExchangeFactory` to instantiate exchange-specific implementations, though only Deribit is currently implemented.

**Concurrent Processing**: Each exchange spawns a tokio task that:
- Polls the market data stream
- Updates Redis with latest data
- Publishes to Kafka topic
- Handles errors with logging (TODO: retry/circuit breaker)

**Health Checks**: Each collector exposes a health check HTTP endpoint (configurable port, default 8080).

**Graceful Shutdown**: Collectors listen for SIGINT (Ctrl+C) to shut down cleanly.

## Deribit Integration

The `deribit-rs/` subdirectory is a workspace member providing the Deribit exchange client. See `deribit-rs/CLAUDE.md` for detailed documentation on:
- WebSocket API client architecture
- Request/response patterns
- Subscription channel implementations
- Testing requirements (needs API credentials)

The main project uses this library in `src/exchanges/deribit/deribit.rs` to:
- Connect to Deribit WebSocket
- Subscribe to order book, trades, and ticker channels
- Parse Deribit-specific message formats into unified `MarketData` types

## Market Data Types

The system handles three primary market data types (enum `MarketData`):

1. **Orderbook**: Bid/ask levels with price and quantity
2. **Trade**: Executed trades with price, quantity, side, timestamp
3. **Ticker**: Summary statistics (last price, 24h volume, best bid/ask, etc.)

Data is keyed by `<exchange>.<symbol>` for both Kafka partitioning and Redis storage.

**Redis TTLs**:
- Orderbook: 3 seconds (high frequency updates)
- Trade: 60 seconds
- Ticker: 300 seconds (5 minutes)

## Current Status & TODOs

**Incomplete Implementation Areas**:
- `src/exchanges/deribit/deribit.rs` - Skeleton implementation only, not functional
- `src/bin/*_collector.rs` - Reference undefined types (KafkaConsumer, ExchangeFactory, etc.)
- Symbol subscription management - Channel-based dynamic symbol updates not implemented
- Retry logic and circuit breakers for error handling
- Cancellation tokens for graceful task shutdown
- ClickHouse integration (producer writes, schema management)

**Working Components**:
- Infrastructure setup via Docker Compose
- Configuration structure (TOML-based)
- Error type definitions
- Basic Redis and Kafka producer structure

## Development Notes

- **Workspace Dependencies**: Shared dependencies defined in root `Cargo.toml` `[workspace.dependencies]`
- **Async Runtime**: Uses tokio with "full" features
- **Logging**: `tracing` crate with structured logging, JSON format in production
- **Serialization**: `serde` and `serde_json` for all data interchange
- **Kafka Client**: `rdkafka` with tokio integration and cmake-build feature
- **WebSocket**: Handled by `deribit-rs` library (tokio-tungstenite with rustls)

## Common Development Commands

```bash
# Check for compilation errors without building
cargo check

# Run clippy for linting
cargo clippy

# Watch mode for development (requires cargo-watch)
cargo watch -x 'run --bin orderbook_collector'

# Build for specific binary
cargo build --bin trades_collector

# View dependency tree
cargo tree
```

## Environment Variables

For development, create a `.env` file in the project root (gitignored):

```env
# Deribit API credentials (required for authenticated endpoints)
DERIBIT_KEY=your_api_key
DERIBIT_SECRET=your_api_secret

# Logging
RUST_LOG=info,market_data=debug
RUST_BACKTRACE=1

# Override default config values if needed
KAFKA_BOOTSTRAP_SERVERS=localhost:29092
REDIS_URL=redis://127.0.0.1:6379
```

## Observability

- **Structured Logging**: All logs are JSON-formatted and sent to Loki via the configured endpoint
- **Grafana Dashboards**: Access at http://localhost:3000 (admin/admin) with pre-configured Loki and ClickHouse data sources
- **Health Checks**: Each collector exposes `/health` endpoint on configured port (default 8080)
- **Metrics**: TODO - Prometheus metrics not yet implemented

## Testing Strategy

- **Unit Tests**: For individual components (parsers, data transformations)
- **Integration Tests**: For deribit-rs library (require API credentials)
- **End-to-End Tests**: TODO - Not yet implemented, would test full data pipeline from exchange to Kafka/Redis

When adding tests:
- Use `#[tokio::test]` for async tests
- Mock external dependencies where possible
- deribit-rs tests must run sequentially: `cargo test -- --test-threads=1`
