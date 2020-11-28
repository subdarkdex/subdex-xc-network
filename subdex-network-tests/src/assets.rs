use codec::{Decode, Encode};
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, Member};
use std::fmt::Debug;
use substrate_subxt::{balances::*, module, system::*, Call, Event, Store};

#[module]
pub trait Assets: System + Balances {
    type AssetId: Parameter + Member + AtLeast32Bit + Default + Copy;
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct IssueCall<T: Assets> {
    #[codec(compact)]
    pub total: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct IssuedEvent<T: Assets> {
    pub asset_id: T::AssetId,
    /// Account balance was transfered to.
    pub from: <T as System>::AccountId,
    /// Amount of balance that was transfered.
    pub amount: T::Balance,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct BalancesStore<'a, T: Assets> {
    #[store(returns = T::Balance)]
    pub balance_of: (T::AssetId, &'a T::AccountId),
}
