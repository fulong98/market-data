#!/bin/bash

QUERY=${1:-"SHOW TABLES FROM market_data"}

echo "🗄️  ClickHouse Query"
echo "===================="
echo ""
echo "Query: $QUERY"
echo ""

docker exec -it market-data-clickhouse clickhouse-client \
    --database market_data \
    --query "$QUERY" \
    --format PrettyCompact

echo ""
echo "💡 Example queries:"
echo "   ./scripts/clickhouse-query.sh \"SELECT count() FROM trades\""
echo "   ./scripts/clickhouse-query.sh \"SELECT * FROM ticker_latest LIMIT 10\""
echo "   ./scripts/clickhouse-query.sh \"SELECT instrument_name, max(timestamp) FROM ticker GROUP BY instrument_name\""
