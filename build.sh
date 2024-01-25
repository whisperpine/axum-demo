#!/bin/bash

docker build -t axum-demo . \
    --platform linux/amd64,linux/arm64 \
    --pull
