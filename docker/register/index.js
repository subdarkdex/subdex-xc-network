#!/usr/bin/env node

const { Keyring } = require('@polkadot/keyring');
const { ApiPromise, WsProvider } = require("@polkadot/api");
const path = require("path")
const fs = require("fs")

async function main () {
  const ip = process.argv[2];
  const port = process.argv[3];
  const runtimePath = process.argv[4];
  const genesisState = process.argv[5];
  const paraId = process.argv[6];
  
  const runtimeFile = fs.readFileSync(path.resolve(__dirname, runtimePath)).toString();
  const genesisFile = fs.readFileSync(path.resolve(__dirname, genesisState)).toString();

  //console.log("runtime: ", runtimeFile);
  console.log("genesisState: ", genesisFile);
  const wsProvider = new WsProvider(`ws://${ip}:${port}`);

  const api = await ApiPromise.create({ 
    provider: wsProvider,
    types: { 
      "Address": "AccountId", 
      "LookupSource": "AccountId",
    }
  });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice');

  api.tx.sudo
    .sudo(api.tx.parasSudoWrapper
      .sudoScheduleParaInitialize(
        paraId,
        {
		"genesisHead": genesisFile, 
		"validationCode": runtimeFile,
		"parachain": true
	},
      )
    )
    .signAndSend(alice, ({ status, events }) => {
	    if (status.isInBlock) {
        console.log('Included at block hash', status.asInBlock.toHex());
        console.log('Events:');

        events.forEach(({ event: { data, method, section }, phase }) => {
          console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
        });
      } else if (status.isFinalized) {
        console.log('Finalized block hash', status.asFinalized.toHex());

        process.exit(0);
      }
    });
}


main().catch(console.error)
