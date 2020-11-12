#[cfg(test)]
mod test {
    use codec::{Compact, Encode};
    use sp_core::crypto;
    use sp_core::crypto::Ss58Codec;
    use sp_keyring::AccountKeyring;
    use std::time::Duration;
    use substrate_subxt::{
        balances, system, system::AccountStoreExt, Call, Client, ClientBuilder, DefaultNodeRuntime,
        EventsDecoder, KusamaRuntime, NodeTemplateRuntime, PairSigner, Runtime,
    };
    use tokio::time::sleep;

    const GENERIC_CHAIN_WS: &str = "ws://127.0.0.1:7744";
    const SUBDEX_CHAIN_WS: &str = "ws://127.0.0.1:9944";
    const RELAY_ALICE_WS: &str = "ws://127.0.0.1:6644";
    const GENERIC_ACCOUNT: &str = "5Ec4AhP7HwJNrY2CxEcFSy1BuqAY3qxvCQCfoois983TTxDA";
    const SUBDEX_ACCOUNT: &str = "5Ec4AhPTL6nWnUnw58QzjJvFd3QATwHA3UJnvSD4GVSQ7Gop";
    const RELAY_ACCOUNT: &str = "5Dvjuthoa1stHkMDTH8Ljr9XaFiVLYe4f9LkAQLDjL3KqHoX";

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
                IssueCall::<NodeTemplateRuntime> {
                    total: Compact(initial_amount),
                },
                &from,
            )
            .await;
        println! {"Issue Call Extrinsic {:?}", r};

        // cannot use assert, need to figure out how to add type assetId here
        // but it actually does get into the block
        let para_transfer_to_relay = generic_client
            .watch(
                TransferToRelayCall::<NodeTemplateRuntime> {
                    dest: to.clone(),
                    amount: transfer_amount,
                    asset_id: Some(0),
                },
                &from,
            )
            .await;
        println! {"Transfer Call Extrinsic {:?}", para_transfer_to_relay};

        //ideally we want to know relay_chain has emitted an event before checking
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

    //     let generic_transfer = generic_client
    //         .watch(
    //             balances::TransferCall {
    //                 to: &relay_account,
    //                 amount: 2 * transfer_amount,
    //             },
    //             &para_admin,
    //         )
    //         .await;
    //     println!(
    //         "generic transfer to relay account on generic {:?}",
    //         generic_transfer
    //     );

    //     let to_pre = generic_client.account(&to, None).await.unwrap();
    //     println! {"pre-account balance {:?}", to_pre};

    //     let relay_parachain_transfer = relay_client
    //         .watch(
    //             TransferToParaCall::<KusamaRuntime> {
    //                 to: 100,
    //                 amount: transfer_amount,
    //                 remark: asset_id,
    //             },
    //             &from,
    //         )
    //         .await;
    //     println! {"Transfer Call Extrinsic {:?}", relay_parachain_transfer};
    //     sleep(Duration::from_millis(6000)).await;

    //     let to_post = generic_client.account(&to, None).await.unwrap();
    //     println! {"post-account balance {:?}", to_post};

    //     assert_eq!(to_pre.data.free + transfer_amount, to_post.data.free);
    // }
    #[derive(Encode)]
    pub struct IssueCall<T: system::System + balances::Balances> {
        total: Compact<T::Balance>,
    }

    impl Call<NodeTemplateRuntime> for IssueCall<NodeTemplateRuntime> {
        const MODULE: &'static str = "Assets";
        const FUNCTION: &'static str = "issue";
        fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
    }

    #[derive(Encode)]
    pub struct TransferToRelayCall<T: system::System + balances::Balances> {
        dest: T::AccountId,
        amount: T::Balance,
        asset_id: Option<u64>,
    }

    impl Call<NodeTemplateRuntime> for TransferToRelayCall<NodeTemplateRuntime> {
        const MODULE: &'static str = "TokenDealer";
        const FUNCTION: &'static str = "transfer_tokens_to_relay_chain";
        fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
    }

    #[derive(Encode)]
    pub struct TransferToParaCall<T: system::System + balances::Balances> {
        /// ParaId
        to: u32,
        amount: T::Balance,
        remark: [u8; 32],
    }

    impl Call<KusamaRuntime> for TransferToParaCall<KusamaRuntime> {
        const MODULE: &'static str = "Parachains";
        const FUNCTION: &'static str = "transfer_to_parachain";
        fn events_decoder(_decoder: &mut EventsDecoder<KusamaRuntime>) {}
    }
}
