use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use std::fmt::Debug;
use substrate_subxt::{balances::*, module, system::*, Call, Event};

#[module]
pub trait DexXCMP: System + Balances {}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferAssetBalanceToParachainChainCall<T: DexXCMP> {
    pub dest: T::AccountId,
    pub amount: T::Balance,
    pub asset_id: Option<T::AssetId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferBalanceToRelayCall<T: DexXCMP> {
    pub para_id: u32,
    pub dest: T::AccountId,
    pub des_asset_id: Option<T::AssetId>,
    pub amount: T::Balance,
    pub asset_id: Option<T::AssetId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredTokensFromRelayChainEvent<T: TokenDealer> {
    pub recieve_local: T::AccountId,
    pub amount: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredAssetViaXCMPEvent<T: DexXCMP> {
    pub para_id: u32,
    pub para_asset_id: Option<T::AssetId>,
    pub recieve_local: T::AccountId,
    pub dex_asset_id: Option<T::AssetId>,
    pub amount: T::Balance,
}
