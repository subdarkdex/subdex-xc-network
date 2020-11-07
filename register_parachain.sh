#!/usr/bin/env bash

set -e

../generic-parachain/target/debug/parachain-collator export-genesis-state --parachain-id=100 gc_gs
../subdex-parachain/target/debug/parachain-collator export-genesis-state --parachain-id=200 sc_gs

cd ./docker/register
yarn 
cd ../..

node ./docker/register \
    127.0.0.1 6644 \
    ../../../generic-parachain/target/debug/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    ../../gc_gs \
    100

node ./docker/register \
    127.0.0.1 6644 \
    ../../../subdex-parachain/target/debug/wbuild/parachain-runtime/parachain_runtime.compact.wasm \
    ../../sc_gs \
    200
