use crate::assets;
use crate::assets::AssetsEventsDecoder;
use codec::{Compact, Decode, Encode};
use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member};
use std::fmt::Debug;
use substrate_subxt::{
    balances, balances::BalancesEventsDecoder, module, system, system::SystemEventsDecoder, Call,
    Event, EventsDecoder,
};

#[module]
pub trait TokenDealer: system::System + balances::Balances + assets::Assets {}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferTokensToRelayChainCall<T: TokenDealer> {
    pub dest: T::AccountId,
    pub amount: T::Balance,
    pub asset_id: Option<T::AssetId>,
}

/// TransferredTokensToRelayChain(AccountId, Option<AssetId>, AccountId, Balance),
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct TransferredTokensToRelayChainEvent<T: TokenDealer> {
    pub from_local: T::AccountId,
    pub asset_id: Option<T::AssetId>,
    /// Account balance was transfered to.
    pub to: T::AccountId,
    /// Amount of balance that was transfered.
    pub amount: T::Balance,
}
