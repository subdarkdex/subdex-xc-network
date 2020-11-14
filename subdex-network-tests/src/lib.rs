use codec::{Codec, Compact, Decode, Encode};
use frame_support::Parameter;
use sp_core::crypto;
use sp_core::crypto::Ss58Codec;
use sp_keyring::AccountKeyring;
use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member};
use std::fmt::Debug;
use std::time::Duration;
use substrate_subxt::{
    balances,
    balances::{BalancesEventsDecoder, TransferCallExt, TransferEvent},
    module, system,
    system::{AccountStoreExt, SystemEventsDecoder},
    Call, Client, ClientBuilder, DefaultNodeRuntime, Event, EventSubscription, EventsDecoder,
    KusamaRuntime, NodeTemplateRuntime, PairSigner, Runtime,
};

mod assets;
mod parachains;
mod token_dealer;

#[cfg(test)]
mod test {
    use super::*;
    // use tokio::time::sleep;

    const GENERIC_CHAIN_WS: &str = "ws://127.0.0.1:7744";
    const SUBDEX_CHAIN_WS: &str = "ws://127.0.0.1:9944";
    const RELAY_ALICE_WS: &str = "ws://127.0.0.1:6644";
    const GENERIC_ACCOUNT: &str = "5Ec4AhP7HwJNrY2CxEcFSy1BuqAY3qxvCQCfoois983TTxDA";
    const SUBDEX_ACCOUNT: &str = "5Ec4AhPTL6nWnUnw58QzjJvFd3QATwHA3UJnvSD4GVSQ7Gop";
    const RELAY_ACCOUNT: &str = "5Dvjuthoa1stHkMDTH8Ljr9XaFiVLYe4f9LkAQLDjL3KqHoX";

    impl parachains::Parachains for KusamaRuntime {}
    impl token_dealer::TokenDealer for NodeTemplateRuntime {}
    impl assets::Assets for NodeTemplateRuntime {
        type AssetId = u64;
    }

    #[tokio::test]
    async fn transfer_tokens_to_relay_chain() {
        let from = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
        let relay_admin = PairSigner::<KusamaRuntime, _>::new(AccountKeyring::Alice.pair());
        let to = AccountKeyring::Bob.to_account_id();
        let initial_amount = 100_000_000_000_000u128;
        let transfer_amount = 10_000_000_000_000u128;
        let generic_para_account = crypto::AccountId32::from_string(GENERIC_ACCOUNT).unwrap();

        let generic_client = ClientBuilder::<NodeTemplateRuntime>::new()
            .set_url(GENERIC_CHAIN_WS)
            .build()
            .await
            .unwrap();

        let relay_client = ClientBuilder::<KusamaRuntime>::new()
            .set_url(RELAY_ALICE_WS)
            .build()
            .await
            .unwrap();

        let relay_transfer = relay_client
            .submit(
                balances::TransferCall {
                    to: &generic_para_account,
                    amount: 2 * transfer_amount,
                },
                &relay_admin,
            )
            .await;
        println!("relay transfer to para account {:?}", relay_transfer);

        let to_pre = relay_client.account(&to, None).await.unwrap();
        println! {"pre-account balance {:?}", to_pre};

        let r = generic_client
            .watch(
                assets::IssueCall::<NodeTemplateRuntime> {
                    total: initial_amount,
                },
                &from,
            )
            .await;
        assert! {r.is_ok()};

        // cannot use assert, need to figure out how to add type Option<AssetId> here
        // but it actually does get into the block
        let para_transfer_to_relay = generic_client
            .watch(
                token_dealer::TransferTokensToRelayChainCall::<NodeTemplateRuntime> {
                    dest: to.clone(),
                    amount: transfer_amount,
                    asset_id: Some(0),
                },
                &from,
            )
            .await;
        println! {"Transfer Call Extrinsic {:?}", para_transfer_to_relay};

        //ideally we want to know relay_chain has emitted an event before checking
        let sub = relay_client.subscribe_events().await.unwrap();
        let mut decoder = EventsDecoder::<KusamaRuntime>::new(relay_client.metadata().clone());
        decoder.with_balances();
        let mut sub = EventSubscription::<KusamaRuntime>::new(sub, decoder);
        sub.filter_event::<TransferEvent<_>>();
        loop {
            let raw = sub.next().await.unwrap().unwrap();
            let event = TransferEvent::<KusamaRuntime>::decode(&mut &raw.data[..]);
            if let Ok(e) = event {
                println!("Balance transfer success: value: {:?}", e.amount);
                if e.amount == transfer_amount {
                    break;
                }
            } else {
                println!("Failed to subscribe to Balances::Transfer Event");
            }
        }
        let to_post = relay_client.account(&to, None).await.unwrap();
        println! {"post-account balance {:?}", to_post};

        assert_eq!(to_pre.data.free + transfer_amount, to_post.data.free);
    }

    // #[tokio::test]
    // async fn transfer_currency_from_relay_chain() {
    //     let from = PairSigner::<KusamaRuntime, _>::new(AccountKeyring::Alice.pair());
    //     let to = AccountKeyring::Alice.to_account_id();
    //     let para_admin = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Bob.pair());
    //     let transfer_amount = 100_000_000_000_000u128;
    //     let relay_account = crypto::AccountId32::from_string(RELAY_ACCOUNT).unwrap();
    //     let asset_id = [0; 32];

    //     let generic_client = ClientBuilder::<NodeTemplateRuntime>::new()
    //         .set_url(GENERIC_CHAIN_WS)
    //         .build()
    //         .await
    //         .unwrap();

    //     let relay_client = ClientBuilder::<KusamaRuntime>::new()
    //         .set_url(RELAY_ALICE_WS)
    //         .build()
    //         .await
    //         .unwrap();

    //     let sub = generic_client.subscribe_events().await.unwrap();
    //     let mut decoder =
    //         EventsDecoder::<NodeTemplateRuntime>::new(generic_client.metadata().clone());
    //     decoder.with_balances();
    //     let mut sub = EventSubscription::<NodeTemplateRuntime>::new(sub, decoder);
    //     sub.filter_event::<TransferEvent<_>>();

    //     let generic_transfer = generic_client
    //         .watch(
    //             balances::TransferCall {
    //                 to: &relay_account,
    //                 amount: 2 * transfer_amount,
    //             },
    //             &para_admin,
    //         )
    //         .await;
    //     assert! {generic_transfer.is_ok()};
    //     println!("Preset: Transfer some balance to relay_chain account on generic OK!",);

    //     let to_pre = generic_client.account(&to, None).await.unwrap();
    //     println! {"pre-account free balance {:?}", to_pre.data.free};

    //     let relay_parachain_transfer = relay_client
    //         .watch(
    //             parachains::TransferToParachainCall::<KusamaRuntime> {
    //                 to: 100,
    //                 amount: transfer_amount,
    //                 remark: asset_id,
    //             },
    //             &from,
    //         )
    //         .await;
    //     assert! {relay_parachain_transfer.is_ok()};

    //     loop {
    //         match sub.next().await {
    //             Some(next_event) => match next_event {
    //                 Ok(raw) => {
    //                     // Only transfer events filtered through
    //                     let e = TransferEvent::<NodeTemplateRuntime>::decode(&mut &raw.data[..])
    //                         .unwrap();
    //                     println!("Balance transfer success: value: {:?}", e.amount);
    //                     if e.amount == transfer_amount {
    //                         break;
    //                     }
    //                 }
    //                 Err(e) => {
    //                     // This happens to the TransferredTokensFromRelayChain event
    //                     println!("Extrinsic err");
    //                     println!("{:?}", e);
    //                     break;
    //                 }
    //             },
    //             None => break,
    //         }
    //     }

    //     let to_post = generic_client.account(&to, None).await.unwrap();
    //     println! {"post-account free balance {:?}", to_post.data.free};
    //     assert_eq!(to_pre.data.free + transfer_amount, to_post.data.free);
    // }
}
