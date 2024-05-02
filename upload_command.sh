#!/bin/bash

if [ -z $DISCORD_TOKEN ]; then
  echo "DISCORD_TOKEN is not set"
  exit 1
fi

if [ -z $DISCORD_APPLICATION_ID ]; then
  echo "DISCORD_APPLICATION_ID is not set"
  exit 1
fi

curl -X PUT -H "Authorization: Bot $DISCORD_TOKEN" -H "Content-Type: application/json" \
  https://discord.com/api/v9/applications/$DISCORD_APPLICATION_ID/commands \
  -d @commands.json | jq
