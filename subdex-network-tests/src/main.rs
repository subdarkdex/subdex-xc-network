use codec::{Compact,Encode};
use sp_core::crypto;
use frame_system as System;
use sp_keyring::AccountKeyring;
use substrate_subxt::{
    balances,
    Call, ClientBuilder, EventsDecoder, KusamaRuntime, NodeTemplateRuntime, PairSigner,
};

const GENERIC_CHAIN_WS: &str = "127.0.0.1:7744";
const SUBDEX_CHAIN_WS: &str = "127.0.0.1:9944";
const RELAY_ALICE_WS: &str = "127.0.0.1:6644";

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Signer for the extrinsic
    let signer = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
    let to = AccountKeyring::Bob.to_account_id();
    // API client, default to connect to 127.0.0.1:9944
    let client = ClientBuilder::<NodeTemplateRuntime>::new()
        .set_url(GENERIC_CHAIN_WS)
        .build()
        .await?;

    // Begin to submit extrinsics
    let transfer_to_relay = client
        .watch(
            TransferCall<NodeTemplateRuntime>{
            },
            &signer,
        )
        .await?;
    println!("\nResult for ipfs_add_bytes: {:?}", transfer_to_relay);

    Ok(())
}

#[derive(Encode)]
pub struct TransferCall<P: Public> {
    dest: P,
    amount: u128,  
    asset_id: Vec<u8>,
}

impl Call<NodeTemplateRuntime> for TransferCall::<P> {
    const MODULE: &'static str = "TokenDealer";
    const FUNCTION: &'static str = "transfer_tokens_to_relay_chain";
    fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
}
