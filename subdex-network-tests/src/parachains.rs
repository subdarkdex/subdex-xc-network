use codec::{Codec, Compact, Decode, Encode};
use frame_support::Parameter;
use substrate_subxt::{
    balances, balances::BalancesEventsDecoder, module, system, system::SystemEventsDecoder, Call,
    Event, EventsDecoder,
};

#[module]
pub trait Parachains: system::System + balances::Balances {}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferToParachainCall<T: Parachains> {
    /// ParaId
    pub to: u32,
    pub amount: T::Balance,
    pub remark: [u8; 32],
}
