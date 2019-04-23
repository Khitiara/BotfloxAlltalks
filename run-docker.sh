#!/bin/sh

if [ "$1" == "--build" ]
then
    docker build -t robotbrain/botflox-alltalks .
    docker rmi -f $(docker images -q --filter "dangling=true")
fi
docker run -d robotbrain/botflox-alltalks