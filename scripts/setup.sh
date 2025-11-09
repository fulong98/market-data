#!/bin/bash

# Create Kafka topic
docker exec -it $(docker ps -q -f name=kafka) \
  kafka-topics --create --topic logs-topic --partitions 1 --replication-factor 1 --bootstrap-server localhost:9092

# Create ClickHouse table (if not created by app)
docker exec -it $(docker ps -q -f name=clickhouse) \
  clickhouse-client --query="CREATE DATABASE IF NOT EXISTS logs"