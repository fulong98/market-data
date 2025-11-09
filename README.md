# Market Data Collection System

A high-performance, real-time cryptocurrency market data collection and streaming system written in Rust. The system ingests live market data from cryptocurrency exchanges (currently Deribit) and streams it to Kafka topics while maintaining the latest snapshots in Redis for low-latency access.

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- **Real-time Data Collection**: WebSocket-based streaming from cryptocurrency exchanges
- **Multi-Data Type Support**: Orderbook, trades, and ticker data collection
- **Dual Storage Architecture**:
  - Kafka for persistent event streaming
  - Redis for low-latency snapshot access
- **Time-Series Storage**: ClickHouse integration for historical data analysis
- **Observability**: Full logging pipeline with Loki/Grafana
- **High Performance**: Async/await architecture using Tokio runtime
- **Modular Design**: Extensible exchange connector framework

## Architecture

```
┌─────────────┐
│  Exchanges  │  (Deribit WebSocket API)
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────┐
│  Collector Binaries             │
│  • orderbook_collector          │
│  • trades_collector             │
│  • ticker_collector             │
└──────────┬──────────────────────┘
           │
           ├──────────────┬────────────────┐
           ▼              ▼                ▼
    ┌──────────┐   ┌──────────┐   ┌────────────┐
    │  Kafka   │   │  Redis   │   │ ClickHouse │
    │ (Stream) │   │ (Cache)  │   │  (History) │
    └──────────┘   └──────────┘   └────────────┘
           │
           ▼
    ┌──────────────┐
    │   Grafana    │ (Observability)
    └──────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70 or higher
- Docker and Docker Compose
- CMake (required for rdkafka)

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/market-data.git
cd market-data
```

### 2. Start Infrastructure Services

```bash
# Start all infrastructure services (Kafka, Redis, ClickHouse, Grafana, etc.)
docker-compose up -d

# Check service health
./scripts/status.sh

# Initialize Kafka topics and ClickHouse database
./scripts/setup.sh
```

Infrastructure services will be available at:
- **Kafka**: `localhost:29092`
- **Kafka UI**: http://localhost:8080
- **Redis**: `localhost:6379`
- **ClickHouse HTTP**: http://localhost:8123
- **ClickHouse Native**: `localhost:9000`
- **Grafana**: http://localhost:3000 (admin/admin)
- **Loki**: http://localhost:3100

### 3. Configure the Application

Create a `.env` file in the project root (optional, for overriding defaults):

```env
# Deribit API credentials (required for authenticated endpoints)
DERIBIT_KEY=your_api_key
DERIBIT_SECRET=your_api_secret

# Logging
RUST_LOG=info,market_data=debug
RUST_BACKTRACE=1

# Infrastructure (optional overrides)
KAFKA_BOOTSTRAP_SERVERS=localhost:29092
REDIS_URL=redis://127.0.0.1:6379
```

Configuration is managed via `config/default.toml`. See [Configuration](#configuration) section for details.

### 4. Build and Run

```bash
# Build the entire workspace
cargo build --release

# Run specific collectors
cargo run --release --bin orderbook_collector
cargo run --release --bin trades_collector
cargo run --release --bin ticker_collector
```

## Project Structure

```
market-data/
├── deribit-rs/           # Standalone Deribit API client library
│   ├── src/
│   │   ├── api/          # API endpoints
│   │   ├── models/       # Data models
│   │   └── websocket/    # WebSocket client
│   └── CLAUDE.md         # Deribit library documentation
├── src/
│   ├── bin/              # Collector binaries
│   │   ├── orderbook_collector.rs
│   │   ├── trades_collector.rs
│   │   └── ticker_collector.rs
│   ├── exchanges/        # Exchange connector implementations
│   │   ├── deribit/
│   │   └── mod.rs        # Exchange trait definition
│   ├── infra/            # Infrastructure integrations
│   │   ├── kafka_producer.rs
│   │   ├── kafka_consumer.rs
│   │   ├── redis.rs
│   │   └── clickhouse.rs
│   ├── models/           # Core data models
│   ├── config.rs         # Configuration management
│   ├── errors.rs         # Error types
│   └── lib.rs
├── config/               # Configuration files
│   └── default.toml
├── scripts/              # Utility scripts
│   ├── setup.sh          # Initialize infrastructure
│   ├── start.sh          # Start services
│   ├── stop.sh           # Stop services
│   ├── status.sh         # Check service status
│   └── ...
├── docker-compose.yml    # Infrastructure services
├── Cargo.toml            # Workspace configuration
└── README.md
```

## Configuration

The application uses TOML-based configuration in `config/default.toml`:

```toml
[kafka]
bootstrap_servers = "localhost:29092"
topics.orderbook = "market.orderbook"
topics.trades = "market.trades"
topics.ticker = "market.ticker"

[redis]
url = "redis://127.0.0.1:6379"
ttl.orderbook = 3        # seconds
ttl.trade = 60
ttl.ticker = 300

[clickhouse]
url = "http://localhost:8123"
database = "market_data"
username = "default"
password = "password"

[exchanges.deribit]
name = "deribit"
ws_url = "wss://www.deribit.com/ws/api/v2"
symbols = ["BTC-PERPETUAL", "ETH-PERPETUAL"]

[logging]
level = "info"
format = "json"
```

## Development

### Building

```bash
# Full workspace build
cargo build

# Release build
cargo build --release

# Build specific binary
cargo build --bin orderbook_collector

# Check without building
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Test deribit-rs library (requires API credentials)
cd deribit-rs
DERIBIT_KEY=xxx DERIBIT_SECRET=xxx cargo test -- --test-threads=1 --nocapture

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings

# Watch mode for development (requires cargo-watch)
cargo install cargo-watch
cargo watch -x 'run --bin orderbook_collector'
```

## Utilities Scripts

The `scripts/` directory contains helpful utilities:

```bash
# Start all infrastructure
./scripts/start.sh

# Stop all services
./scripts/stop.sh

# Check service status
./scripts/status.sh

# View logs
./scripts/logs.sh [service_name]

# Restart services
./scripts/restart.sh

# List Kafka topics
./scripts/kafka-topics.sh

# Query ClickHouse
./scripts/clickhouse-query.sh "SELECT * FROM trades LIMIT 10"

# Check Redis data
./scripts/redis-check.sh

# Clean up all data
./scripts/clean.sh
```

## Observability

### Logging

All applications use structured JSON logging with the `tracing` crate:

```bash
# View aggregated logs in Grafana
open http://localhost:3000

# Query logs with LogQL
{container_name="market-data-orderbook"}
```

### Health Checks

Each collector exposes a health check endpoint:

```bash
# Default port 8080
curl http://localhost:8080/health
```

### Metrics (TODO)

Prometheus metrics integration is planned but not yet implemented.

## Data Flow

1. **Collector connects** to exchange WebSocket API
2. **Subscribes** to market data channels (orderbook/trades/ticker)
3. **Parses** exchange-specific messages into unified `MarketData` types
4. **Dual write**:
   - **Redis**: Updates latest snapshot with TTL
   - **Kafka**: Publishes event to topic
5. **ClickHouse** consumer (TODO) writes to time-series tables

### Market Data Types

```rust
pub enum MarketData {
    Orderbook(OrderbookData),  // Bid/ask levels
    Trade(TradeData),          // Executed trades
    Ticker(TickerData),        // Price summary stats
}
```

Keys use format: `<exchange>.<symbol>` (e.g., `deribit.BTC-PERPETUAL`)

## Workspace Structure

This project uses Cargo workspaces:

- **deribit-rs**: Independent Deribit API client library
- **market-data**: Main application with collector binaries

Shared dependencies are defined in the root `Cargo.toml` `[workspace.dependencies]`.

## Technology Stack

- **Runtime**: Tokio (async/await)
- **WebSocket**: tokio-tungstenite + rustls
- **Serialization**: serde + serde_json
- **Kafka**: rdkafka
- **Redis**: redis-rs with async support
- **Database**: ClickHouse (time-series)
- **Logging**: tracing + tracing-subscriber
- **Config**: config crate (TOML)
- **Error Handling**: thiserror + anyhow

## Current Status

**This project is in early development.**

**Working Components**:
- Infrastructure setup (Docker Compose)
- Configuration structure
- Error type definitions
- Basic Kafka/Redis integration structure
- deribit-rs library foundation

**In Progress / TODO**:
- [ ] Complete Deribit connector implementation
- [ ] Implement collector binaries
- [ ] Symbol subscription management
- [ ] ClickHouse producer and schema
- [ ] Retry logic and circuit breakers
- [ ] Graceful shutdown with cancellation tokens
- [ ] Prometheus metrics
- [ ] End-to-end integration tests
- [ ] Performance benchmarks

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Deribit API Documentation](https://docs.deribit.com/)
- Inspired by market data systems at quantitative trading firms

## Support

For issues, questions, or contributions, please open an issue on GitHub.