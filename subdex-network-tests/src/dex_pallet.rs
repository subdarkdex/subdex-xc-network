use codec::{Decode, Encode};
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, Member};
use std::fmt::Debug;
use substrate_subxt::{balances::*, module, system::*, Call, Store};

#[module]
pub trait DexPallet: System + Balances {
    type AssetId: Parameter + Member + AtLeast32Bit + Default + Copy;
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Asset<AssetId: Default + Debug + Ord + Copy> {
    MainNetworkCurrency,
    ParachainAsset(AssetId),
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AssetBalancesStore<'a, T: DexPallet> {
    #[store(returns = T::Balance)]
    pub account_id: &'a T::AccountId,
    pub asset_id: T::AssetId,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct InitializeExchangeCall<T: DexPallet> {
    pub first_asset: Asset<T::AssetId>,
    pub first_asset_amount: T::Balance,
    pub second_asset: Asset<T::AssetId>,
    pub second_asset_amount: T::Balance,
}
