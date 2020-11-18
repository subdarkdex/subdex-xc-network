mod assets;
mod dex_pallet;
mod dex_xcmp;
mod parachains;
mod token_dealer;

#[cfg(test)]
mod test {
    use super::*;
    use codec::{Decode, Encode};
    use sp_core::crypto;
    use sp_core::crypto::Ss58Codec;
    use sp_keyring::AccountKeyring;
    use sp_std::convert::TryInto;
    use std::time::Duration;
    use substrate_subxt::{
        balances,
        balances::{BalancesEventsDecoder, TransferEvent},
        system::AccountStoreExt,
        ClientBuilder, EventSubscription, EventsDecoder, KusamaRuntime, NodeTemplateRuntime,
        PairSigner,
    };
    use tokio::time::sleep;

    const GENERIC_CHAIN_WS: &str = "ws://127.0.0.1:7744";
    const SUBDEX_CHAIN_WS: &str = "ws://127.0.0.1:9944";
    const RELAY_ALICE_WS: &str = "ws://127.0.0.1:6644";
    const GENERIC_ACCOUNT: &str = "5Ec4AhP7HwJNrY2CxEcFSy1BuqAY3qxvCQCfoois983TTxDA";
    const SUBDEX_ACCOUNT: &str = "5Ec4AhPTL6nWnUnw58QzjJvFd3QATwHA3UJnvSD4GVSQ7Gop";
    //const RELAY_ACCOUNT: &str = "5Dvjuthoa1stHkMDTH8Ljr9XaFiVLYe4f9LkAQLDjL3KqHoX";
    const SUBDEX_PARA_ID: u32 = 200;
    const GENERIC_PARA_ID: u32 = 100;

    impl parachains::Parachains for KusamaRuntime {}
    impl token_dealer::TokenDealer for NodeTemplateRuntime {
        type AssetIdOf = u64;
        type ParaId = u32;
    }
    impl assets::Assets for NodeTemplateRuntime {
        type AssetId = u64;
    }
    impl dex_pallet::DexPallet for NodeTemplateRuntime {
        type AssetId = u64;
    }
    impl dex_xcmp::DexXCMP for NodeTemplateRuntime {
        type AssetIdOf = u64;
        type ParaId = u32;
    }

    fn encoded_to_remark(mut v: Vec<u8>) -> [u8; 32] {
        v.resize(32, 0);
        let boxed_slice = v.into_boxed_slice();
        let boxed_array: Box<[u8; 32]> = match boxed_slice.try_into() {
            Ok(ba) => ba,
            Err(o) => panic!("Expected a Vec of length {} but it was {}", 32, o.len()),
        };
        *boxed_array
    }

    #[tokio::test]
    async fn transfer_tokens_between_dex_and_relay_chains() {
        let charlie_account = AccountKeyring::Charlie.to_account_id();
        let charlie_generic_pair =
            PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Charlie.pair());
        let charlie_relay_pair =
            PairSigner::<KusamaRuntime, _>::new(AccountKeyring::Charlie.pair());

        let transfer_amount = 10_000_000_000_000u128;

        let generic_client = ClientBuilder::<NodeTemplateRuntime>::new()
            .set_url(SUBDEX_CHAIN_WS)
            .build()
            .await
            .unwrap();

        let relay_client = ClientBuilder::<KusamaRuntime>::new()
            .set_url(RELAY_ALICE_WS)
            .build()
            .await
            .unwrap();

        println!("----- Running transfer currency and tokens from Relay to Para chain -----");

        let charlie_asset_pre = generic_client
            .fetch(
                dex_pallet::AssetBalancesStore {
                    account_id: &charlie_account,
                    asset_id: 0,
                },
                None,
            )
            .await
            .unwrap()
            .unwrap_or(0);
        println! {"charlie generic asset account free balance before transfers {:?}", charlie_asset_pre};

        let relay_transfer_asset = relay_client
            .watch(
                parachains::TransferToParachainCall::<KusamaRuntime> {
                    to: SUBDEX_PARA_ID,
                    amount: transfer_amount,
                    remark: encoded_to_remark(vec![0; 32]),
                },
                &charlie_relay_pair,
            )
            .await;
        assert! {relay_transfer_asset.is_ok()};
        println! {"Transfer Asset from Relay is OK"};

        println!("Ensuring block after transfer event...");
        sleep(Duration::from_millis(9000)).await;

        let charlie_asset_post = generic_client
            .fetch(
                dex_pallet::AssetBalancesStore {
                    account_id: &charlie_account,
                    asset_id: 0,
                },
                None,
            )
            .await
            .unwrap()
            .unwrap_or(0);
        println! {"charlie generic asset account free balance after transfers {:?}", charlie_asset_post};

        // TODO fail
        //assert_eq!(charlie_asset_pre + transfer_amount, charlie_asset_post);

        println!("----- Success! transfer currency from Relay to Para chain -----");
        println!();
        println!();
        println!();

        println!("----- Running transfer currency from Para to Relay chain -----");

        let charlie_relay_pre = relay_client.account(&charlie_account, None).await.unwrap();
        println! {"charlie relay account free balance before transfers: {:?}", charlie_relay_pre.data.free};

        let transfer_currency_to_relay = generic_client
            .watch(
                dex_xcmp::TransferBalanceToRelayChainCall {
                    dest: &charlie_account,
                    amount: transfer_amount,
                },
                &charlie_generic_pair,
            )
            .await;
        assert!(transfer_currency_to_relay.is_ok());
        println! {"Transfer currency to Relay is OK"};

        let charlie_relay_post = relay_client.account(&charlie_account, None).await.unwrap();
        println! {"charlie relay account free balance after transfers: {:?}", charlie_relay_post.data.free};

        assert_eq!(
            charlie_relay_pre.data.free + transfer_amount,
            charlie_relay_post.data.free
        );

        println!("----- Success! transfer currency and tokens from Para to Relay chain -----");
    }

    // #[tokio::test]
    // async fn transfer_tokens_between_generic_and_relay_chains() {
    //     let alice_account = AccountKeyring::Alice.to_account_id();
    //     let alice_generic_pair =
    //         PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
    //     let alice_relay_pair = PairSigner::<KusamaRuntime, _>::new(AccountKeyring::Alice.pair());
    //     let generic_para_account = crypto::AccountId32::from_string(GENERIC_ACCOUNT).unwrap();

    //     let asset_issue_amount = 50_000_000_000_000u128;
    //     let transfer_amount = 10_000_000_000_000u128;

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

    //     println!("----- Running transfer currency and tokens from Para to Relay chain -----");
    //     // Initialise so generic para has balance on relay chain
    //     let relay_transfer = relay_client
    //         .watch(
    //             balances::TransferCall {
    //                 to: &generic_para_account,
    //                 amount: asset_issue_amount,
    //             },
    //             &alice_relay_pair,
    //         )
    //         .await;
    //     assert!(relay_transfer.is_ok());
    //     println!("Preset: Relay transfer to para account on relay chain OK",);

    //     // Initialise so we have some generic assets
    //     let issue_asset = generic_client
    //         .watch(
    //             assets::IssueCall::<NodeTemplateRuntime> {
    //                 total: asset_issue_amount,
    //             },
    //             &alice_generic_pair,
    //         )
    //         .await;
    //     assert! {issue_asset.is_ok()};
    //     let e = assets::IssuedEvent::<NodeTemplateRuntime>::decode(
    //         &mut &issue_asset.unwrap().events[0].data[..],
    //     )
    //     .unwrap();
    //     println!(
    //         "Preset: Issue some token is OK! New asset_id {:?}",
    //         e.asset_id
    //     );
    //     let issued_asset_id = e.asset_id;

    //     let alice_relay_pre = relay_client.account(&alice_account, None).await.unwrap();
    //     println! {"Alice relay account free balance before transfers: {:?}", alice_relay_pre.data.free};

    //     let transfer_currency_to_relay = generic_client
    //         .watch(
    //             token_dealer::TransferTokensToRelayChainCall::<NodeTemplateRuntime> {
    //                 dest: alice_account.clone(),
    //                 amount: transfer_amount,
    //                 asset_id: None,
    //             },
    //             &alice_generic_pair,
    //         )
    //         .await;
    //     assert!(transfer_currency_to_relay.is_ok());
    //     println! {"Transfer currency to Relay is OK"};

    //     let transfer_asset_to_relay = generic_client
    //         .watch(
    //             token_dealer::TransferTokensToRelayChainCall::<NodeTemplateRuntime> {
    //                 dest: alice_account.clone(),
    //                 amount: transfer_amount,
    //                 asset_id: Some(issued_asset_id),
    //             },
    //             &alice_generic_pair,
    //         )
    //         .await;
    //     assert!(transfer_asset_to_relay.is_ok());
    //     println! {"Transfer asset to Relay is OK"};

    //     let alice_relay_post = relay_client.account(&alice_account, None).await.unwrap();
    //     println! {"Alice relay account free balance after transfers: {:?}", alice_relay_post.data.free};

    //     assert_eq!(
    //         alice_relay_pre.data.free + (2 * transfer_amount),
    //         alice_relay_post.data.free
    //     );

    //     println!("----- Success! transfer currency and tokens from Para to Relay chain -----");
    //     println!();
    //     println!();
    //     println!();

    //     println!("----- Running transfer currency and tokens from Relay to Para chain -----");

    //     let alice_asset_pre = generic_client
    //         .fetch(
    //             assets::BalancesStore {
    //                 balance_of: (issued_asset_id, &alice_account),
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     println! {"Alice generic asset account free balance before transfers {:?}", alice_asset_pre};

    //     let remark = Some(issued_asset_id).encode();
    //     let relay_transfer_asset = relay_client
    //         .watch(
    //             parachains::TransferToParachainCall::<KusamaRuntime> {
    //                 to: 100,
    //                 amount: transfer_amount / 2,
    //                 remark: encoded_to_remark(remark),
    //             },
    //             &alice_relay_pair,
    //         )
    //         .await;
    //     assert! {relay_transfer_asset.is_ok()};
    //     println! {"Transfer Asset from Relay is OK"};

    //     let relay_transfer_currency = relay_client
    //         .watch(
    //             parachains::TransferToParachainCall::<KusamaRuntime> {
    //                 to: 100,
    //                 amount: transfer_amount / 2,
    //                 remark: [0u8; 32],
    //             },
    //             &alice_relay_pair,
    //         )
    //         .await;
    //     assert! {relay_transfer_currency.is_ok()};
    //     println! {"Transfer Currency from Relay is OK"};

    //     let sub = generic_client.subscribe_events().await.unwrap();
    //     let mut decoder =
    //         EventsDecoder::<NodeTemplateRuntime>::new(generic_client.metadata().clone());
    //     decoder.with_balances();
    //     let mut sub = EventSubscription::<NodeTemplateRuntime>::new(sub, decoder);
    //     // TODO filter TransferredTokensFromRelayChainEvent also
    //     sub.filter_event::<TransferEvent<_>>();
    //     while let Some(next_event) = sub.next().await {
    //         match next_event {
    //             Ok(raw) => {
    //                 // Only transfer events filtered through
    //                 let e =
    //                     TransferEvent::<NodeTemplateRuntime>::decode(&mut &raw.data[..]).unwrap();
    //                 println!("Currency Balance transfer success: value: {:?}", e.amount);
    //                 if e.amount == transfer_amount / 2 {
    //                     break;
    //                 }
    //             }
    //             Err(e) => {
    //                 println!("Extrinsic err");
    //                 println!("{:?}", e);
    //             }
    //         }
    //     }

    //     println!("Ensuring block after transfer event...");
    //     sleep(Duration::from_millis(6000)).await;

    //     let alice_asset_post = generic_client
    //         .fetch(
    //             assets::BalancesStore {
    //                 balance_of: (issued_asset_id, &alice_account),
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     println! {"Alice generic asset account free balance after transfers {:?}", alice_asset_post};

    //     assert_eq!(alice_asset_pre + (transfer_amount / 2), alice_asset_post);

    //     println!("----- Success! transfer currency and tokens from Relay to Para chain -----");
    //     println!();
    //     println!();
    //     println!();
    // }

    // #[tokio::test]
    // async fn transfer_tokens_between_generic_and_dex_chain() {
    //     let bob_account = AccountKeyring::Bob.to_account_id();
    //     let bob_pair = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Bob.pair());
    //     let generic_para_account = crypto::AccountId32::from_string(GENERIC_ACCOUNT).unwrap();
    //     let dex_para_account = crypto::AccountId32::from_string(SUBDEX_ACCOUNT).unwrap();

    //     let asset_issue_amount = 50_000_000_000_000u128;
    //     let transfer_amount = 10_000_000_000_000u128;

    //     let generic_client = ClientBuilder::<NodeTemplateRuntime>::new()
    //         .set_url(GENERIC_CHAIN_WS)
    //         .build()
    //         .await
    //         .unwrap();

    //     let dex_client = ClientBuilder::<NodeTemplateRuntime>::new()
    //         .set_url(SUBDEX_CHAIN_WS)
    //         .build()
    //         .await
    //         .unwrap();

    //     // Initialise so we have some generic assets
    //     let issue_asset = generic_client
    //         .watch(
    //             assets::IssueCall::<NodeTemplateRuntime> {
    //                 total: asset_issue_amount,
    //             },
    //             &bob_pair,
    //         )
    //         .await;
    //     assert! {issue_asset.is_ok()};
    //     let e = assets::IssuedEvent::<NodeTemplateRuntime>::decode(
    //         &mut &issue_asset.unwrap().events[0].data[..],
    //     )
    //     .unwrap();
    //     println!(
    //         "Preset: Issue some token is OK! New asset_id {:?}",
    //         e.asset_id
    //     );
    //     let issued_asset_id = e.asset_id;

    //     let dex_currency_id = dex_client
    //         .fetch(
    //             dex_xcmp::AssetIdByParaAssetIdStore {
    //                 para_id: GENERIC_PARA_ID,
    //                 asset_id: None,
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap();

    //     let bob_currency_pre = match dex_currency_id {
    //         Some(currency_id) => dex_client
    //             .fetch(
    //                 dex_pallet::AssetBalancesStore {
    //                     account_id: &bob_account,
    //                     asset_id: currency_id,
    //                 },
    //                 None,
    //             )
    //             .await
    //             .unwrap()
    //             .unwrap(),
    //         None => 0,
    //     };
    //     println! {"Bob account free balance before transfers: {:?}", bob_currency_pre};
    //     let bob_asset_pre = 0;
    //     println! {"Bob account new asset balance before transfers always 0"};

    //     let transfer_asset_to_dex = generic_client
    //         .watch(
    //             token_dealer::TransferAssetsToParachainChainCall::<NodeTemplateRuntime> {
    //                 para_id: SUBDEX_PARA_ID,
    //                 dest: bob_account.clone(),
    //                 des_asset_id: Some(issued_asset_id),
    //                 amount: transfer_amount,
    //                 asset_id: Some(issued_asset_id),
    //             },
    //             &bob_pair,
    //         )
    //         .await;
    //     println! {"Transfer Asset to Dex is OK if error is Codec {:?}", transfer_asset_to_dex};
    //     // assert!(transfer_asset_to_dex.is_ok());// some decoding of event does not work

    //     let transfer_currency_to_dex = generic_client
    //         .watch(
    //             token_dealer::TransferAssetsToParachainChainCall::<NodeTemplateRuntime> {
    //                 para_id: SUBDEX_PARA_ID,
    //                 dest: bob_account.clone(),
    //                 des_asset_id: None,
    //                 amount: transfer_amount,
    //                 asset_id: None,
    //             },
    //             &bob_pair,
    //         )
    //         .await;
    //     assert!(transfer_currency_to_dex.is_ok());
    //     println! {"Transfer Currency to Dex is OK"};

    //     // May want to listen to events here...
    //     println!("Ensure Extrinsics Included...");
    //     sleep(Duration::from_millis(9000)).await;

    //     let dex_asset_id = dex_client
    //         .fetch(
    //             dex_xcmp::AssetIdByParaAssetIdStore {
    //                 para_id: GENERIC_PARA_ID,
    //                 asset_id: Some(issued_asset_id),
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();

    //     // Can avoid re-fetch if Some()
    //     let dex_currency_id = dex_client
    //         .fetch(
    //             dex_xcmp::AssetIdByParaAssetIdStore {
    //                 para_id: GENERIC_PARA_ID,
    //                 asset_id: None,
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();

    //     let bob_currency_post = dex_client
    //         .fetch(
    //             dex_pallet::AssetBalancesStore {
    //                 account_id: &bob_account,
    //                 asset_id: dex_currency_id,
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     println! {"Bob account free balance after transfers: {:?}", bob_currency_post};

    //     let bob_asset_post = dex_client
    //         .fetch(
    //             dex_pallet::AssetBalancesStore {
    //                 account_id: &bob_account,
    //                 asset_id: dex_asset_id,
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     println! {"Bob asset balance after transfers: {:?}", bob_asset_post};

    //     assert_eq!(bob_asset_pre + transfer_amount, bob_asset_post);
    //     assert_eq!(bob_currency_pre + transfer_amount, bob_currency_post);
    //     println!("----- Success! transfer currency and tokens from Generic to Dex chain -----");
    //     println!();
    //     println!();
    //     println!();

    //     println!("----- Running transfer currency and tokens from Relay to Para chain -----");

    //     let bob_currency_pre = generic_client.account(&bob_account, None).await.unwrap();
    //     println! {"Bob account free balance before transfers: {:?}", bob_currency_pre.data.free};

    //     let bob_asset_pre = generic_client
    //         .fetch(
    //             assets::BalancesStore {
    //                 balance_of: (issued_asset_id, &bob_account),
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     println! {"Bob generic asset account free balance before transfers {:?}", bob_asset_pre};

    //     // Does not get recieved on generic
    //     let dex_transfer_asset = dex_client
    //         .watch(
    //             dex_xcmp::TransferAssetBalanceToParachainChainCall {
    //                 para_id: GENERIC_PARA_ID,
    //                 dest: &bob_account,
    //                 para_asset_id: Some(issued_asset_id),
    //                 amount: transfer_amount,
    //             },
    //             &bob_pair,
    //         )
    //         .await;
    //     assert! {dex_transfer_asset.is_ok()};
    //     println! {"Transfer Asset from Dex is OK"};

    //     // Does not get recieved on generic
    //     let dex_transfer_currency = dex_client
    //         .watch(
    //             dex_xcmp::TransferAssetBalanceToParachainChainCall {
    //                 para_id: GENERIC_PARA_ID,
    //                 dest: &bob_account,
    //                 para_asset_id: None,
    //                 amount: transfer_amount,
    //             },
    //             &bob_pair,
    //         )
    //         .await;
    //     assert! {dex_transfer_currency.is_ok()};
    //     println! {"Transfer Currency from Dex is OK"};

    //     println!("Ensuring block after transfer event...");
    //     sleep(Duration::from_millis(15000)).await;

    //     let bob_currency_post = generic_client.account(&bob_account, None).await.unwrap();
    //     println! {"Bob account free balance after transfers: {:?}", bob_currency_post.data.free};

    //     let bob_asset_post = generic_client
    //         .fetch(
    //             assets::BalancesStore {
    //                 balance_of: (issued_asset_id, &bob_account),
    //             },
    //             None,
    //         )
    //         .await
    //         .unwrap()
    //         .unwrap();
    //     println! {"Bob generic asset account balance after transfers {:?}", bob_asset_post};

    //     assert_eq!(bob_asset_pre + transfer_amount, bob_asset_post);
    //     assert_eq!(
    //         bob_currency_pre.data.free + transfer_amount,
    //         bob_currency_post.data.free
    //     );

    //     println!("----- Success! transfer currency and tokens from Dex to Generic chain -----");
    // }
}
