#!/bin/bash

echo "📋 Kafka Topics"
echo "==============="
echo ""

# List all topics
echo "Existing topics:"
docker exec market-data-kafka kafka-topics --bootstrap-server localhost:9092 --list

echo ""
echo "Topic details:"
for topic in market-data-orderbook market-data-trades market-data-ticker; do
    echo ""
    echo "📊 $topic:"
    docker exec market-data-kafka kafka-topics --bootstrap-server localhost:9092 --describe --topic "$topic" 2>/dev/null || echo "  (not created yet)"
done

echo ""
echo "💡 To consume from a topic:"
echo "   docker exec -it market-data-kafka kafka-console-consumer \\"
echo "     --bootstrap-server localhost:9092 \\"
echo "     --topic market-data-ticker \\"
echo "     --from-beginning"
