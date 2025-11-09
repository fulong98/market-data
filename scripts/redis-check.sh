#!/bin/bash

echo "🔴 Redis Status"
echo "==============="
echo ""

# Check Redis connectivity
echo "Connection test:"
docker exec market-data-redis redis-cli ping

echo ""
echo "📊 Key statistics:"
docker exec market-data-redis redis-cli INFO stats | grep -E "total_commands_processed|instantaneous_ops_per_sec"

echo ""
echo "🔑 Sample keys (latest market data):"
docker exec market-data-redis redis-cli --scan --pattern "deribit:*" | head -10

echo ""
echo "💡 To inspect a specific key:"
echo "   docker exec market-data-redis redis-cli GET \"deribit:BTC-PERPETUAL:ticker\""
