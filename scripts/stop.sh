#!/bin/bash
set -e

echo "🛑 Stopping Market Data infrastructure..."

docker-compose down

echo "✅ All services stopped!"
echo ""
echo "💡 To remove all data volumes, run:"
echo "   docker-compose down -v"
