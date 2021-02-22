#!/usr/bin/env bash

set -e

../../cumulus/target/debug/rococo-collator export-genesis-state --parachain-id=100 tmp/rc_gs
../../cumulus/target/debug/rococo-collator export-genesis-wasm tmp/rc_wasm

# ../generic-parachain/target/release/parachain-collator export-genesis-state --parachain-id=100 tmp/gc_gs
# ../subdex-parachain/target/release/parachain-collator export-genesis-state --parachain-id=200 tmp/sc_gs

 cd ./docker/register
 yarn 
 cd ../..
 
 node ./docker/register \
     127.0.0.1 6644 \
     ../../tmp/rc_wasm \
     ../../tmp/rc_gs \
     100

# node ./docker/register \
#     127.0.0.1 6644 \
#     ../../../subdex-parachain/target/release/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
#     ../../tmp/sc_gs \
#     200

