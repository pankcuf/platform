/// transformer
pub mod transformer;
/// v0
pub mod v0;

use crate::state_transition_action::identity::identity_credit_transfer::v0::IdentityCreditTransferTransitionActionV0;
use derive_more::From;
use dpp::balances::credits::Credits;
use dpp::platform_value::Identifier;
use dpp::prelude::{IdentityNonce, UserFeeIncrease};

/// action
#[derive(Debug, Clone, From)]
pub enum IdentityCreditTransferTransitionAction {
    /// v0
    V0(IdentityCreditTransferTransitionActionV0),
}

impl IdentityCreditTransferTransitionAction {
    /// Nonce
    pub fn nonce(&self) -> IdentityNonce {
        match self {
            IdentityCreditTransferTransitionAction::V0(transition) => transition.nonce,
        }
    }

    /// Transfer amount
    pub fn transfer_amount(&self) -> Credits {
        match self {
            IdentityCreditTransferTransitionAction::V0(transition) => transition.transfer_amount,
        }
    }

    /// Identity Id
    pub fn identity_id(&self) -> Identifier {
        match self {
            IdentityCreditTransferTransitionAction::V0(transition) => transition.identity_id,
        }
    }

    /// Recipient Id
    pub fn recipient_id(&self) -> Identifier {
        match self {
            IdentityCreditTransferTransitionAction::V0(transition) => transition.recipient_id,
        }
    }

    /// fee multiplier
    pub fn user_fee_increase(&self) -> UserFeeIncrease {
        match self {
            IdentityCreditTransferTransitionAction::V0(transition) => transition.user_fee_increase,
        }
    }
}
