use codec::Encode;
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, Member};
use std::fmt::Debug;
use substrate_subxt::{balances::*, module, system::*, Store};

#[module]
pub trait DexPallet: System + Balances {
    type AssetId: Parameter + Member + AtLeast32Bit + Default + Copy;
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AssetBalancesStore<'a, T: DexPallet> {
    #[store(returns = T::Balance)]
    pub account_id: &'a T::AccountId,
    pub asset_id: T::AssetId,
}
