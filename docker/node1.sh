#!/usr/bin/env bash

# Exit on failure
set -ex

echo "start run node1"
echo `date +"%Y-%m-%d %H:%M:%S.%z"`
echo "RUST_MIN_STACK:$RUST_MIN_STACK"

#WORKDIR /ccm

nohup ./run/ccm-node \
--chain=local \
--base-path /ccm/run/data/node1 \
--name alice \
--node-key=213216127a8a8756f4017d2aaafa7e0054e9958e7ded5d5784c5e2f6f6365e0f \
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
--bootnodes /dns/node2/tcp/30333/p2p/12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH \
--bootnodes /dns/node3/tcp/30333/p2p/12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ \
--bootnodes /dns/node4/tcp/30333/p2p/12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN \
> /ccm/run/logs/ccm-node1.log 2>&1 &

echo $! > /ccm/run/logs/node1.pid

tail -f /ccm/run/logs/ccm-node1.log
