use codec::{Codec, Compact, Decode, Encode};
use frame_support::Parameter;
use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member};
use std::fmt::Debug;
use substrate_subxt::{
    balances, balances::BalancesEventsDecoder, module, system, system::SystemEventsDecoder, Call,
    Event, EventsDecoder,
};

#[module]
pub trait Assets: system::System + balances::Balances {
    type AssetId: Parameter + Member + AtLeast32Bit + Default + Copy;
}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct IssueCall<T: Assets> {
    #[codec(compact)]
    pub total: T::Balance,
}

/// TransferredTokensToRelayChain(AccountId, Option<AssetId>, AccountId, Balance),
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct IssuedEvent<T: Assets> {
    pub asset_id: T::AssetId,
    /// Account balance was transfered to.
    pub from: <T as system::System>::AccountId,
    /// Amount of balance that was transfered.
    pub amount: T::Balance,
}
