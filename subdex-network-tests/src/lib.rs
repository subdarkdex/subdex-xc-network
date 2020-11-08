#[cfg(test)]
mod test {
    use codec::{Compact, Encode};
    use sp_core::crypto;
    use sp_core::crypto::Ss58Codec;
    use sp_keyring::AccountKeyring;
    use substrate_subxt::{
        balances, system, system::AccountStoreExt, Call, Client, ClientBuilder, DefaultNodeRuntime,
        EventsDecoder, KusamaRuntime, NodeTemplateRuntime, PairSigner, Runtime,
    };

    const GENERIC_CHAIN_WS: &str = "ws://127.0.0.1:7744";
    const SUBDEX_CHAIN_WS: &str = "ws://127.0.0.1:9944";
    const RELAY_ALICE_WS: &str = "ws://127.0.0.1:6644";
    const GENERIC_ACCOUNT: &str = "5Ec4AhP7HwJNrY2CxEcFSy1BuqAY3qxvCQCfoois983TTxDA";

    #[tokio::test]
    async fn transfer_tokens_to_relay_chain() {
        let mut signer = PairSigner::<NodeTemplateRuntime, _>::new(AccountKeyring::Alice.pair());
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
                &signer,
            )
            .await;
        println! {"Issue Call Extrinsic {:?}", r};

        // cannot use assert, need to figure out how to add type assetId here
        // but it actually does get into the block
        let transfer = generic_client
            .watch(
                TransferCall::<NodeTemplateRuntime> {
                    dest: to.clone(),
                    amount: transfer_amount,
                    asset_id: Some(0),
                },
                &signer,
            )
            .await;
        println! {"Transfer Call Extrinsic {:?}", transfer};

        //ideally can we say.. oh relay_chain has emitted an event, now check

        let to_post = relay_client.account(&to, None).await.unwrap();
        println! {"post-account balance {:?}", to_post};

        assert_eq!(to_pre.data.free + transfer_amount, to_post.data.free);
    }

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
    pub struct TransferCall<T: system::System + balances::Balances> {
        dest: T::AccountId,
        amount: T::Balance,
        asset_id: Option<u64>,
    }

    impl Call<NodeTemplateRuntime> for TransferCall<NodeTemplateRuntime> {
        const MODULE: &'static str = "TokenDealer";
        const FUNCTION: &'static str = "transfer_tokens_to_relay_chain";
        fn events_decoder(_decoder: &mut EventsDecoder<NodeTemplateRuntime>) {}
    }
}
