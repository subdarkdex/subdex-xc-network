#!/usr/bin/env bash

set -e

VERSION=$1

#Dex-collator
docker build \
    --file ./dex-chain-collator.dockerfile \
    --target collator \
    --tag subdarkdex/subdex-collator:"$VERSION" \
    .

#Generic-collator
docker build \
    --file ./generic-chain-collator.dockerfile \
    --target collator \
    --tag subdarkdex/generic-collator:"$VERSION" \
    .

# Dex-runtime
docker build \
    --file ./dex-chain-collator.dockerfile \
    --target runtime \
    --tag subdarkdex/subdex-runtime:"$VERSION" \
    .

# Generic-runtime
docker build \
    --file ./generic-chain-collator.dockerfile \
    --target runtime \
    --tag subdarkdex/generic-runtime:"$VERSION" \
    .

# Registrar
docker build \
    --file ./parachain-registrar.dockerfile \
    --tag subdarkdex/subdex-registrar:"$VERSION" \
    .

docker rmi -f $(docker images -f "dangling=true" -q)
