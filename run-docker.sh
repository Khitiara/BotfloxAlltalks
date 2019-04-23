#!/bin/sh

if [ "$1" == "--build" ]
then
    docker build -t robotbrain/BotfloxAlltalks .
    docker rmi -f $(docker images -q --filter "dangling=true")
fi
docker run -d robotbrain/BotfloxAlltalks