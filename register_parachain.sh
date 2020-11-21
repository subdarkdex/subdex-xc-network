#!/usr/bin/env bash

set -e

../generic-parachain/target/release/parachain-collator export-genesis-state --parachain-id=100 tmp/gc_gs
../subdex-parachain/target/release/parachain-collator export-genesis-state --parachain-id=200 tmp/sc_gs

cd ./docker/register
yarn 
cd ../..

node ./docker/register \
    127.0.0.1 6644 \
    ../../../generic-parachain/target/release/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    ../../tmp/gc_gs \
    100

node ./docker/register \
    127.0.0.1 6644 \
    ../../../subdex-parachain/target/release/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    ../../tmp/sc_gs \
    200

