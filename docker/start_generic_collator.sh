#!/usr/bin/env bash

# this script runs the generic-parachain-collator after fetching
# appropriate bootnode IDs
#
# this is _not_ a general-purpose script; it is closely tied to the
# root docker-compose-xc.yml

set -e

dc="/usr/local/bin/generic-parachain-collator"

if [ ! -x "$dc" ]; then
    echo "FATAL: no correct executables"
    exit 1
fi

dc_args=( "$@" )

charlie_ip="172.28.1.4"
dave_ip="172.28.1.6"

get_id () {
    ip="$1"
    ./wait-for-it.sh "$ip:9933" -t 100 -- \
        curl -sS \
            -H 'Content-Type: application/json' \
            --data '{"id":1,"jsonrpc":"2.0","method":"system_localPeerId"}' \
            "$ip:9933" |\
    jq -r '.result'
}

bootnode () {
    ip="$1"
    id=$(get_id "$ip")
    if [ -z "$id" ]; then
        echo >&2 "failed to get id for $ip"
        exit 1
    fi
    echo "/ip4/$ip/tcp/30333/p2p/$id"
}



dc_args+=("--base-path=/generic/generic_data" 
    "--parachain-id=100" 
    "--validator"
    "--port=30333"
    "--ws-port=9944"
    "--rpc-port=9933"
    "--unsafe-ws-external" 
    "--unsafe-rpc-external" 
    "--rpc-cors=all" 
    "--out-peers=0" 
    "--in-peers=0" 
    "--" "--chain=/generic/rococo-4.json" 
    "--bootnodes=$(bootnode "$charlie_ip")" 
    "--bootnodes=$(bootnode "$dave_ip")" 
    "--ws-port=6622" 
    "--rpc-port=6611" 
    "--port=40330"
    )

set -x
"$dc" "${dc_args[@]}"
