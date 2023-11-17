#!/bin/bash

# Build docker image and run locally.

red_echo() {
    echo -e "\033[31m$@\033[0m"
}

green_echo() {
    echo -e "\033[32m$@\033[0m"
}

green_echo ":: building image..."
docker build --pull -t axum-demo ../..
if [ $? -ne 0 ]; then
    red_echo ":: failed to build docker image"
    exit 1
fi
docker compose up -d
