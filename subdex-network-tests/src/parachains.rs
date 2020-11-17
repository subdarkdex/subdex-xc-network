use codec::Encode;
use substrate_subxt::{balances::*, module, system::*, Call};

#[module]
pub trait Parachains: System + Balances {}

#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct TransferToParachainCall<T: Parachains> {
    /// ParaId
    pub to: u32,
    pub amount: T::Balance,
    pub remark: [u8; 32],
}
