#!/bin/sh

echo "start run ccm chain node session cluster every 20s"
#set -x   #print every run cmd

echo "check RUST_MIN_STACK: $RUST_MIN_STACK"

#export RUST_LOG="info"

echo "start node1"

./ccm-node-session \
--chain=local \
--base-path /tmp/ccm/node1 \
--name alice \
--node-key=213216127a8a8756f4017d2aaafa7e0054e9958e7ded5d5784c5e2f6f6365e0f \
--port 30333 \
--ws-port 9948 \
--rpc-port 9933 \
--validator \
--rpc-cors all \
--execution native \
--unsafe-rpc-external \
--unsafe-ws-external \
--rpc-methods=unsafe \
-lruntime=debug \
--bootnodes /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH \
--bootnodes /ip4/127.0.0.1/tcp/30335/p2p/12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ \
--bootnodes /ip4/127.0.0.1/tcp/30336/p2p/12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN \
> ccm-log1.out 2>&1 &

echo $! > "node1.pid"

sleep 20
echo "start node2"

./ccm-node-session \
--chain=local \
--base-path /tmp/ccm/node2 \
--name bob \
--node-key=3b7a5239d28e90a6941dba9266b8f7135b966885e21988fb0c6e7f55516c73f3 \
--port 30334 \
--ws-port 9947 \
--rpc-port 9934 \
--validator \
--rpc-cors all \
--execution native \
--unsafe-rpc-external \
--unsafe-ws-external \
--rpc-methods=unsafe \
-lruntime=debug \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE \
--bootnodes /ip4/127.0.0.1/tcp/30335/p2p/12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ \
--bootnodes /ip4/127.0.0.1/tcp/30336/p2p/12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN \
> ccm-log2.out 2>&1 &

echo $! > "node2.pid"

sleep 20
echo "start node3"

./ccm-node-session \
--chain=local \
--base-path /tmp/ccm/node3 \
--name charlie \
--node-key=3a9d5b35b9fb4c42aafadeca046f6bf56107bd2579687f069b42646684b94d9e \
--port 30335 \
--ws-port=9946 \
--rpc-port 9935 \
--validator \
--rpc-cors all \
--execution native \
--unsafe-rpc-external \
--unsafe-ws-external \
--rpc-methods=unsafe \
-lruntime=debug \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE  \
--bootnodes /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH \
--bootnodes /ip4/127.0.0.1/tcp/30336/p2p/12D3KooWPHWFrfaJzxPnqnAYAoRUyAHHKqACmEycGTVmeVhQYuZN \
> ccm-log3.out 2>&1 &

echo $! > "node3.pid"

sleep 20
echo "start node4"

./ccm-node-session \
--chain=local \
--base-path /tmp/ccm/node4 \
--name dave  \
--node-key=a99331ff4f0e0a0434a6263da0a5823ea3afcfffe590c9f3014e6cf620f2b19a \
--port 30336 \
--ws-port=9944 \
--rpc-port 9936 \
--pruning archive \
--rpc-cors all \
--execution native \
--unsafe-rpc-external \
--unsafe-ws-external \
--rpc-methods=unsafe \
 -lruntime=debug \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE \
--bootnodes /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH \
--bootnodes /ip4/127.0.0.1/tcp/30335/p2p/12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ \
> ccm-log4.out 2>&1 &

echo $! > "node4.pid"

echo "DONE"

exit 0