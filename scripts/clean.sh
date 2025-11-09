#!/bin/bash
set -e

echo "🧹 Cleaning Market Data infrastructure..."
echo ""
echo "⚠️  WARNING: This will:"
echo "   - Stop all services"
echo "   - Remove all containers"
echo "   - Delete all data volumes (Redis, Kafka, ClickHouse, Loki, Grafana)"
echo ""
read -p "Are you sure? (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "❌ Cancelled"
    exit 0
fi

echo ""
echo "🛑 Stopping services..."
docker-compose down -v

echo ""
echo "🗑️  Removing orphaned volumes..."
docker volume prune -f --filter "label=com.docker.compose.project=market-data"

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "💡 To start fresh, run: ./scripts/start.sh"
