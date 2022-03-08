#!/bin/sh

# Exit on failure
set -ex

echo "start run node3" `date +"%Y-%m-%d %H:%M:%S.%z"`

#WORKDIR /ccm

nohup ./run/ccm-node \
--chain=local \
--base-path ./run/data/node3 \
--name node3 \
--node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e \
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
--bootnodes /dns/node4/tcp/30333/p2p/12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN \
> /ccm/run/logs/ccm-node3.log 2>&1 &

echo $! > /ccm/run/logs/node3.pid

tail -f /ccm/run/logs/ccm-node3.log
