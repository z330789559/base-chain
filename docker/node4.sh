#!/bin/sh

# Exit on failure
set -ex

echo "start run node4"
echo `date +"%Y-%m-%d %H:%M:%S.%z"`

#WORKDIR /ccm

nohup ./run/ccm-node \
--chain=local \
--base-path ./run/data/node4 \
--name node4 \
--node-key=a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a \
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
--bootnodes /dns/node2/tcp/30333/p2p/12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH \
--bootnodes /dns/node3/tcp/30333/p2p/12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ \
> /ccm/run/logs/ccm-node4.log 2>&1 &

echo $! > /ccm/run/logs/node4.pid

tail -f /ccm/run/logs/ccm-node4.log
