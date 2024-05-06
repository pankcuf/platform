use crate::identity::identity_public_key::SecurityLevel;
use crate::identity::identity_public_key::KeyID;
use crate::state_transition::state_transitions::identity::identity_credit_transfer_transition::IdentityCreditTransferTransition;
use crate::state_transition::StateTransitionIdentitySigned;

impl StateTransitionIdentitySigned for IdentityCreditTransferTransition {
    fn signature_public_key_id(&self) -> KeyID {
        match self {
            IdentityCreditTransferTransition::V0(transition) => {
                transition.signature_public_key_id()
            }
        }
    }

    fn set_signature_public_key_id(&mut self, key_id: KeyID) {
        match self {
            IdentityCreditTransferTransition::V0(transition) => {
                transition.set_signature_public_key_id(key_id)
            }
        }
    }

    fn security_level_requirement(&self) -> Vec<SecurityLevel> {
        match self {
            IdentityCreditTransferTransition::V0(transition) => {
                transition.security_level_requirement()
            }
        }
    }
}
