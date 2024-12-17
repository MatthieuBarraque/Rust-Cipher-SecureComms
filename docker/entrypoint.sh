#!/bin/bash

if [ "$RUN_MODE" = "server" ]; then
    echo "Starting server..."
    exec /app/server
elif [ "$RUN_MODE" = "client" ]; then
    echo "Starting client..."
    exec /app/client
else
    echo "Please specify RUN_MODE=server or RUN_MODE=client"
    exit 1
fi