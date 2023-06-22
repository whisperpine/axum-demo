#!/bin/bash

# Build docker image and run locally.

echo "building image..."
docker build --pull -t axum-demo ../..
docker compose up -d
