#!/bin/bash

# Build docker image and run locally.

echo ":: building image..."
docker build --pull -t axum-demo ../..
if [ $? -ne 0 ]; then
    echo ":: failed to build docker image"
    exit 1
fi
docker compose up -d
