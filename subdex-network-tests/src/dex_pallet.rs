use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use std::fmt::Debug;
use substrate_subxt::{balances::*, module, system::*, Call, Event};

#[module]
pub trait DexXCMP: System + Balances {}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct AssetBalancesStore<'a, T: DexPallet> {
    #[store(returns = T::Balance)]
    pub balance_of: (&'a T::AccountId, T::AssetId),
}
