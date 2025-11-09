#!/bin/bash

SERVICE=${1:-}

if [ -z "$SERVICE" ]; then
    echo "📜 Showing logs for all services (Ctrl+C to exit)..."
    docker-compose logs -f
else
    echo "📜 Showing logs for $SERVICE (Ctrl+C to exit)..."
    docker-compose logs -f "market-data-$SERVICE" 2>/dev/null || docker-compose logs -f "$SERVICE"
fi
