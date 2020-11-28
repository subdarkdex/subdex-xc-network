use crate::assets::*;
use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, Member};
use std::fmt::Debug;
use substrate_subxt::{balances::*, module, system::*, Call, Event};

#[module]
pub trait TokenDealer: System + Balances + Assets {
    type AssetIdOf: Parameter + Member + AtLeast32Bit + Default + Copy;
    type ParaId: Parameter + Member + AtLeast32Bit + Default + Copy;
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferTokensToRelayChainCall<T: TokenDealer> {
    pub dest: T::AccountId,
    pub amount: T::Balance,
    pub asset_id: Option<T::AssetId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferAssetsToParachainChainCall<T: TokenDealer> {
    pub para_id: T::ParaId,
    pub dest: T::AccountId,
    pub des_asset_id: Option<T::AssetIdOf>,
    pub amount: T::Balance,
    pub asset_id: Option<T::AssetIdOf>,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredTokensFromRelayChainEvent<T: TokenDealer> {
    pub recieve_local: T::AccountId,
    pub amount: T::Balance,
    pub asset_id: Option<T::AssetId>,
    pub result: DispatchResult,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredTokensViaXCMPEvent<T: TokenDealer> {
    pub para_id: T::ParaId,
    pub recieve_local: T::AccountId,
    pub amount: T::Balance,
    pub para_asset_id: Option<T::AssetId>,
    pub result: DispatchResult,
}
