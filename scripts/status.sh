#!/bin/bash

echo "📊 Market Data Infrastructure Status"
echo "===================================="
echo ""

# Check if any containers are running
if ! docker-compose ps | grep -q "Up"; then
    echo "❌ No services are running"
    echo ""
    echo "To start services, run: ./scripts/start.sh"
    exit 0
fi

# Show container status
docker-compose ps

echo ""
echo "🔍 Health Status:"
docker-compose ps --format json 2>/dev/null | jq -r '.[] | "\(.Service): \(.State) \(.Health)"' || docker-compose ps

echo ""
echo "💾 Volume Usage:"
docker volume ls | grep market-data || echo "No volumes found"

echo ""
echo "🌐 Network:"
docker network ls | grep observability || echo "Network not found"
