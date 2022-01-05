#!/bin/sh

echo "start run ccm chain node1 & node2"
set -x

echo "check RUST_MIN_STACK:"
echo $RUST_MIN_STACK

export RUST_LOG="debug"

/data/ccm/ccmrun/ccm-node \
--base-path /data/ccm/ccmrun/blocks/blocks/cc0 \
--chain /data/ccm/ccmrun/customSpecRaw_test.json \
--port 30333 \
--ws-port 9945 \
--rpc-port 9934 \
--validator \
--name ccm_test_sh_vally_bay_node1 \
--node-key=d972ec7c4d8e59751699b1f632ffdac6457550eaaeac42b4fdb93e2033e18604 \
--pruning archive \
--rpc-cors all \
--execution wasm \
--unsafe-rpc-external \
--unsafe-ws-external \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--bootnodes /ip4/127.0.0.1/tcp/30334/p2p/12D3KooWFsLQAmNCpxgeeXY9mH9DgqgchutwmSL2wqeaAhhbG1R4 \
--rpc-methods=unsafe \
-lruntime=debug > ccm-log2.out 2>&1 &

/data/ccm/ccmrun/ccm-node \
--base-path /data/ccm/ccmrun/blocks/blocks/cc1 \
--chain /data/ccm/ccmrun/customSpecRaw_test.json \
--port 30334 \
--ws-port 9944 \
--rpc-port 9933 \
--validator \
--name ccm_test_sh_vally_bay_node2 \
--node-key=bbd6661b0b87fcb9482ea06f181ff94c9ddf948e7eae84066cc71d0826c293be \
--pruning archive \
--rpc-cors all \
--execution wasm \
--unsafe-rpc-external \
--unsafe-ws-external \
--telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWB6bcZYvH4iRih9HndMGsSYdVMA1xjN1gJyq6ifYxprYN \
--rpc-methods=unsafe \
-lruntime=debug > ccm-log1.out 2>&1 &

echo $RUST_LOG

exit 0