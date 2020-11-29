#!/usr/bin/env bash
set -ex

# build generic-parachain
DIRECTORY='../generic-parachain'
if [ ! -d "$DIRECTORY" ]; then
    git clone https://github.com/subdarkdex/generic-parachain.git ../generic-parachain
fi

cd ../generic-parachain
git fetch --all
git checkout tags/rc5 
cargo build --release
cd ../subdex-xc-network

# build subdex-parachain 
DIRECTORY='../subdex-parachain'
if [ ! -d "$DIRECTORY" ]; then
    git clone https://github.com/subdarkdex/subdex-parachain.git ../subdex-parachain
fi
cd ../subdex-parachain
git fetch --all
# TODO update branch
git checkout master
cargo build --release
cd ../subdex-xc-network
