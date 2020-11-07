#!/usr/bin/env bash

# this script runs the generic-parachain-collator after fetching
# appropriate bootnode IDs
#
# this is _not_ a general-purpose script; it is closely tied to the
# root docker-compose.yml

set -e

gc="../generic-parachain/target/release/parachain-collator"
sc="../subdex-parachain/target/release/parachain-collator"

if [ ! -x "$gc" -o  ! -x "$sc" ]; then
    echo "FATAL: no correct executables"
    exit 1
fi

# name the variable with the incoming args so it isn't overwritten later by function calls
gc_args=( "$@" )
sc_args=( "$@" )

alice_p2p="30333"
bob_p2p="30335"
charlie_p2p="30336"
dave_p2p="30337"
alice_rpc="9933"
bob_rpc="9911"
charlie_rpc="8811"
dave_rpc="8833"


get_id () {
    rpc="$1"
    ./wait-for-it.sh "127.0.0.1:$rpc" -t 10 -- \
        curl \
            -H 'Content-Type: application/json' \
            --data '{"id":1,"jsonrpc":"2.0","method":"system_localPeerId"}' \
            "127.0.0.1:$rpc" |\
    jq -r '.result'
}

bootnode () {
    p2p="$1"
    rpc="$2"
    id=$(get_id "$rpc")
    if [ -z "$id" ]; then
        echo >&2 "failed to get id for $node"
        exit 1
    fi
    echo "/ip4/127.0.0.1/tcp/$p2p/p2p/$id"
}

gc_args+=("--base-path=generic_parachain_data" 
    "--parachain-id=100" 
    "--validator"
    "--ws-port=7744" 
    "--unsafe-ws-external" 
    "--unsafe-rpc-external" 
    "--rpc-cors=all" 
    "--rpc-port=7733" 
    "--port=40444" 
    "--out-peers=0" 
    "--in-peers=0" 
    "--" 
    "--chain=dex_raw.json" 
    "--bootnodes=$(bootnode "$alice_p2p" "$alice_rpc")" 
    "--bootnodes=$(bootnode "$bob_p2p" "$bob_rpc")" 
    "--ws-port=7722"
    "--rpc-port=7711"
    "--port=40334"
    )


sc_args+=("--base-path=subdex_parachain_data" 
    "--parachain-id=200" 
    "--validator"
    "--ws-port=9944" 
    "--unsafe-ws-external" 
    "--unsafe-rpc-external" 
    "--rpc-cors=all" 
    "--rpc-port=6633" 
    "--port=40440" 
    "--out-peers=0" 
    "--in-peers=0" 
    "--" 
    "--chain=dex_raw.json" 
    "--bootnodes=$(bootnode "$charlie_p2p" "$charlie_rpc")" 
    "--bootnodes=$(bootnode "$dave_p2p" "$dave_rpc")" 
    "--ws-port=6622" 
    "--rpc-port=6611" 
    "--port=40330"
    )

set -x
# "$gc" "${gc_args[@]}" & "$sc" "${sc_args[@]}"
"$sc" "${sc_args[@]}" 
