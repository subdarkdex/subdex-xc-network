#!/usr/bin/env bash

# this script runs the polkadot network where the executable polkadot is compiled with 
# cargo build --release --features=real-overseer

set -e

polkadot="../../polkadot/target/release/polkadot"

if [ ! -x "$polkadot" ]; then
    echo "FATAL: no correct executables"
    exit 1
fi

alice_args=( "$@" )
bob_args=( "$@" )
alice_args+=(
    "--tmp" 
    "--chain=./chainspec/rococo_local.json"
    "--port=30333"
    "--ws-port=6644"
    "--rpc-port=9933"
    "--rpc-cors=all"
    "--validator"
    "--alice"
    "--no-prometheus"
    "--no-telemetry"
    )

bob_args+=("--tmp" 
    "--chain=./chainspec/rococo_local.json"
    "--port=30335"
    "--ws-port=9922"
    "--rpc-port=9911"
    "--rpc-cors=all"
    "--validator"
    "--bob"
    "--discover-local"
    "--no-prometheus"
    "--no-telemetry"
    )

set -x
"$polkadot" "${alice_args[@]}" & "$polkadot" "${bob_args[@]}" 
