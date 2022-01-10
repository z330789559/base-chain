#!/bin/sh

echo "start run ccm chain node session cluster every 20s"
#set -x   #print every run cmd

echo "check RUST_MIN_STACK: $RUST_MIN_STACK"

#export RUST_LOG="info"

echo "start node248"

nohup /data/ccm/ccm-node \
--base-path /data/ccm/blocks/cc0 \
--chain /data/ccm/customSpecRaw_prod.json \
--port 10001 --ws-port 9944 --rpc-port 9933 --validator \
--name node0 --pruning archive \
--rpc-cors all \
--node-key=aa46533b8d33b7f4caaf0e8950c9b260060d2484c1468441b46e174a78fdd5ca \
--bootnodes /ip4/152.32.219.101/tcp/10001/p2p/12D3KooWP6UGwYbymnoQ4WUDvL7mRzzK6EobuZFiLgxVGRAqXZsy \
--bootnodes /ip4/107.155.56.71/tcp/10001/p2p/12D3KooWJ4mop8ZZgLeDdBidxbZr2ddguKPMteLGdAGPofsPG33m \
--execution wasm --rpc-methods=Unsafe --unsafe-rpc-external --unsafe-ws-external
> ccm-log248.out 2>&1 &

echo $! > "node248.pid"

echo "DONE"

exit 0