#!/bin/bash
set -e

echo "🚀 Starting Market Data infrastructure..."

# Check if docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker Desktop first."
    exit 1
fi

# Start all services
echo "📦 Starting Docker Compose services..."
docker-compose up -d

echo ""
echo "⏳ Waiting for services to be healthy..."
sleep 5

# Wait for all services to be healthy
services=("redis" "zookeeper" "kafka" "clickhouse" "loki" "grafana")
for service in "${services[@]}"; do
    echo -n "Checking $service... "
    max_attempts=30
    attempt=0
    while [ $attempt -lt $max_attempts ]; do
        if docker-compose ps | grep "market-data-$service" | grep -q "healthy\|Up"; then
            echo "✅"
            break
        fi
        attempt=$((attempt + 1))
        sleep 2
    done
    if [ $attempt -eq $max_attempts ]; then
        echo "⚠️  (timeout, but continuing)"
    fi
done

echo ""
echo "✅ All services started!"
echo ""
echo "📊 Access points:"
echo "  - Grafana:       http://localhost:3000 (admin/admin)"
echo "  - Kafka UI:      http://localhost:8080"
echo "  - ClickHouse:    http://localhost:8123"
echo "  - Loki:          http://localhost:3100"
echo "  - Redis:         localhost:6379"
echo "  - Kafka:         localhost:29092"
echo ""
echo "📝 Next steps:"
echo "  1. Open Grafana at http://localhost:3000"
echo "  2. Navigate to Dashboards -> Market Data Logs"
echo "  3. Run your Rust application to start collecting data"
echo ""
echo "💡 Useful commands:"
echo "  - View logs:     ./scripts/logs.sh [service]"
echo "  - Stop all:      ./scripts/stop.sh"
echo "  - Restart:       ./scripts/restart.sh"
echo "  - Check status:  ./scripts/status.sh"
