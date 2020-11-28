#!/usr/bin/env bash

set -e

cd ./subdex-network-tests
cargo test -- --nocapture test::transfer_tokens_between_dex_and_relay_chains
cargo test -- --nocapture test::transfer_tokens_between_generic_and_relay_chains
cargo test -- --nocapture test::transfer_tokens_between_generic_and_dex_chain
cd ..
