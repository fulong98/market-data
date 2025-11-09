#!/bin/bash
set -e

echo "🔄 Restarting Market Data infrastructure..."

./scripts/stop.sh
sleep 2
./scripts/start.sh
