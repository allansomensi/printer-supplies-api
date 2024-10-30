#!/bin/bash

ENV_FILE=".env"

if [ -f "$ENV_FILE" ]; then
    export $(grep -v '^#' "$ENV_FILE" | xargs)
else
    echo "Error: .env file not found!"
    exit 1
fi

if [ -z "$ENVIRONMENT" ]; then
    echo "Error: ENVIRONMENT variable not set in .env!"
    exit 1
fi

DOTENV_PATH="environments/.env.$ENVIRONMENT"

if [ -f "$DOTENV_PATH" ]; then
    export $(grep -v '^#' "$DOTENV_PATH" | xargs)
else
    echo "Error: Environment file $DOTENV_PATH not found!"
    exit 1
fi
