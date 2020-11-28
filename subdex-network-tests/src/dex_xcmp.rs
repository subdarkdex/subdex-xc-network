use crate::dex_pallet::*;
use codec::{Decode, Encode};
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, Member};
use std::fmt::Debug;
use std::marker::Send;
use substrate_subxt::{balances::*, module, system::*, Call, Event, Store};

#[module]
pub trait DexXCMP: System + Balances + DexPallet {
    type AssetIdOf: Parameter + Member + AtLeast32Bit + Default + Copy;
    type ParaId: Parameter + Member + AtLeast32Bit + Default + Copy;
    type ParaChainAssetId: Member + Default + Copy + Send + Encode + Decode;
    type DexAssetId: Member + Default + Copy + Send + Encode + Decode;
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferAssetBalanceToParachainChainCall<'a, T: DexXCMP> {
    pub para_id: u32,
    pub dest: &'a T::AccountId,
    pub para_asset_id: Option<T::AssetIdOf>,
    pub amount: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferBalanceToRelayChainCall<'a, T: DexXCMP> {
    pub dest: &'a T::AccountId,
    pub amount: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredTokensFromRelayChainEvent<T: DexXCMP> {
    pub recieve_local: T::AccountId,
    pub amount: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredAssetViaXCMPEvent<T: DexXCMP> {
    pub para_id: T::ParaId,
    pub para_asset_id: Option<T::AssetId>,
    pub recieve_local: T::AccountId,
    pub dex_asset_id: Option<T::AssetId>,
    pub amount: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AssetIdByParaAssetIdStore<T: DexXCMP> {
    #[store(returns = T::AssetIdOf)]
    pub para_id: T::ParaId,
    pub asset_id: Option<T::AssetIdOf>,
}
