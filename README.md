# Subdex cross chain network 

This repo provides simple scripts, inspired by polkadot/cumulus, to set up a network with:-
1. Relay Chain with 4 validators (Alice, Bob, Charlie and Dave)
2. Generic parachain (parachain with generic-assets-token-dealer, assets and balances pallets)
3. Subdex parachain (parachain with dex-xcmp and dex-pallet)

The relay chain chain-specs is `rococo-4.json` from cumulus-workshop

This is a part of the submission for Open Grant awarded by the Web3Foundation
1. [subdex-ui](https://github.com/subdarkdex/subdex-ui) (React frontend providing friendly UI)
1. **subdex-xc-network** (current repo)
1. [pallet-subdex]()
1. [pallet-generic-token-dealer](https://github.com/subdarkdex/pallet-generic-token-dealer)
1. Helper Repo - [subdex-parachain](https://github.com/subdarkdex/subdex-parachain) (Parachains using the Cumulus framework with the dex-xcmp and dex-pallet)
1. Helper Repo - [generic-parachain](https://github.com/subdarkdex/generic-parachain) (Parachains using the Cumulus framework with the generic-assets-token-dealer and assets pallet)
____

## Interaction
The subdex testnet can be interacted with through 
- Subdex Parachain (Dex is here!)
- Relay Chain
- Generic Parachain

#### To Connnect to our Deployed Network 
- Subdex Parachain - Please visit our dex UI [here]() to connect to the Subdex Parachain

You can also interact with the Relay Chain and the Generic Parachain through the polkadot.js.org/apps 
- Relay Chain - wss://subdex.link/relay
- Generic Parachain - wss://subdex.link/generic


#### local testnet (deployed with docker-compose-xc.yml) 
- Subdex Parachain - localhost:9944
- Relay Chain - localhost:6644
- Generic Parachain - localhost:7744

#### Types:
If you are using the polkadot.js.org/apps web application, please make sure you add:

```
{
  "AssetId": "u64",
  "AssetIdOf": "AssetId",
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "RefCount": "u8",
}
```

For subdex-parachain specific types, please see [here](https://github.com/subdarkdex/subdex-ui/blob/master/src/config/common.json#L5)

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

## Run a local testnet 

```sh
# in the root of this directory
docker-compose --file docker-compose-xc.yml up

# To stop it and remove all data
docker-compose --file docker-compose-xc.yml down -v
```
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
1. register parachains
1. run subdex and generic parachains


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
### 5. run network tests
```
./run_network_tests.sh
```
### 6. stop validators
`docker-compose -f docker-compose-validatorsOnly.yml down -v`

### 7. (purge-chains, all chains, all genesis states)
```
./clear_all.sh
```

___
### Building all docker images for dockerhub

1. **Base images** - this is to compile the binary / wasm file from branches of subdex_cumulus

```sh
# To build

# same for generic-parachain
git clone https://github.com/subdarkdex/subdex_parachain.git dex-parachain
cd dex-parachain
docker build --tag subdarkdex/subdex-chain:<version>

```

2. **Collators, WASM Runtime Volume, Registrar**
- collators - both dex and generic parachains
- wasm runtime volume - this is a copy of the wasm runtime for the collators, used to register parachain, we also have the genesis state volume built during docker-compose up for this purpose
- registrar - simple polkadotjs cli container to register the parachains using sudo


```sh
cd docker
./build-containers.sh v0.2.0 
# or other versions
```
