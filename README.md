# Subdex cross chain network 

This repo provides simple scripts, inspired by polkadot/cumulus, to set up a network with:-
1. Relay Chain with 4 validators (Alice, Bob, Charlie and Dave)
2. Generic parachain (parachain with generic-assets-token-dealer, assets and balances pallets)
3. Subdex parachain (parachain with dex-xcmp and dex-pallet)

The relay chain chain-specs is a modified version of westend_local, with `validator_count = 4` to support 2 parachains.

This is a part of the submission for Subdex grant awarded by the Web3Foundation
1. [subdex-ui](https://github.com/subdarkdex/subdex-ui) (React frontend providing friendly UI)
1. **subdex-xc-network** (current repo)
1. TODO - individual pallets 
1. Helper Repo - [subdex-parachain](https://github.com/subdarkdex/subdex-parachain) (Parachains using the Cumulus framework with the dex-xcmp and dex-pallet)
1. Helper Repo - [generic-parachain](https://github.com/subdarkdex/generic-parachain) (Parachains using the Cumulus framework with the generic-assets-token-dealer and assets pallet)

___
## Development

## Run local parachain binarys
### Pre-requisits
- Docker version 19.03.8, build afacb8b
- execute access for the `.sh` files in this repo

### Setup for native parachain binaries

*NOTE:* - we are not using the script provided in cumulus because we want 2 parachains, also, need to be able to build and rebuild the parachain binaries as we experiment. But when we get more familiar / more stable versions of the parachains, we can build a similar script to do all steps. 

Steps required are:-
1. set up relay chain validators
1. set up default cumulus parachain 
1. run paraA and paraB
1. register parachains


### 1. Set up validators
`docker-compose -f docker-compose-validatorsOnly.yml up` will set up alice, bob, charlie and dave

### 2. build parachains
```
./build_collators_from_local.sh
```
This will set up 2 repositories parallel to this one if they are not already setup, one is generic-parachain and the other the subdex-chain

### 3. register parachains
```
./register_parachain.sh
```

### 4. run the parachains
```sh
# Run whichever you need
./start_generic_chain.sh
./start_subdex_chain.sh
```

### 5. stop validators
`docker-compose -f docker-compose-validatorsOnly.yml down`

### 6. stop collators
`killall parachain-collator`

### 7. (purge-chains, all chains)
```
./clear_all.sh
```

___
### TODO building all docker images for dockerhub

1. **Base images** - this is to compile the binary / wasm file from branches of subdex_cumulus

```sh
# To build

# for generic-parachain
git clone https://github.com/subdarkdex/subdex_parachains.git dex-parachain
cd dex-parachain
docker build --tag subdarkdex/generic-chain:<version>

```

2. **Collators, WASM Runtime Volume, Registrar**
- collators - both dex and generic parachains
- wasm runtime volume - this is a copy of the wasm runtime for the collators, used to register parachain, we also have the genesis state volume built during docker-compose up for this purpose
- registrar - simple polkadotjs cli container to register the parachains using sudo


```sh
cd docker
./build-containers.sh v0.1.0 
# or other versions
```

#### To run with docker
```sh
# in the root of this directory
docker-compose --file docker-compose-xc.yml up
```

#### To stop
```sh
# in the root of this directory
docker-compose --file docker-compose-xc.yml down -v
./clear_all.sh 
```

### Config
#### Accounts

The parachain account is tied to the `parachain_id` [encoded](https://github.com/paritytech/polkadot/blob/master/parachain/src/primitives.rs#L164)

The relay account is tied to the binary representation of `relay` [here](https://github.com/subdarkdex/generic-parachain/blob/master/pallets/token-dealer/src/lib.rs#L54)
```
 Parachain id: Id(100) Generic Parachain
 Parachain Account: 5Ec4AhP7HwJNrY2CxEcFSy1BuqAY3qxvCQCfoois983TTxDA
... 
 Parachain id: Id(200) Dex Parachain
 Parachain Account: 5Ec4AhPTL6nWnUnw58QzjJvFd3QATwHA3UJnvSD4GVSQ7Gop


 Relay Account on Parachain: 5Dvjuthoa1stHkMDTH8Ljr9XaFiVLYe4f9LkAQLDjL3KqHoX
```
#### Chain specs
The DarkDex chain spec is a duplication of the westend-local chain, but with 4 validators and validator count as 4. Changes were made to v0.8.14 - chain_spec.rs

```sh
# westend-local was updated with 4 validators, Alice, Bob, Charlie and Dave
./target/release/polkadot build-spec --chain=westend-local --raw --disable-default-bootnode > dex_raw.json
```

___
## Interaction

This version works with 1.29 polkadot js on https://polkadot.js.org/apps/

#### Types:
```
{
  "AssetId": "u64",
  "Address": "AccountId",
  "LookupSource": "AccountId"
 
}
```


