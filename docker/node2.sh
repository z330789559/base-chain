#!/bin/sh

# Exit on failure
set -ex

echo "start run node2" `date +"%Y-%m-%d %H:%M:%S.%z"`

#WORKDIR /ccm

nohup ./run/ccm-node \
--chain=local \
--base-path ./run/data/node2 \
--name node2 \
--node-key=3b7a5239d28e90a6941dba9266b8f7135b966885e21988fb0c6e7f55516c73f3 \
--port 30333 \
--ws-port 9944 \
--rpc-port 9933 \
--validator \
--rpc-cors all \
--execution native \
--unsafe-rpc-external \
--unsafe-ws-external \
--rpc-methods=unsafe \
-lruntime=debug \
--bootnodes /dns/node1/tcp/30333/p2p/12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE \
--bootnodes /dns/node3/tcp/30333/p2p/12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ \
--bootnodes /dns/node4/tcp/30333/p2p/12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN \
> /ccm/run/logs/ccm-node2.log 2>&1 &

echo $! > /ccm/run/logs/node2.pid

tail -f /ccm/run/logs/ccm-node2.log