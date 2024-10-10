use crate::identity::identity_public_key::SecurityLevel::{CRITICAL, HIGH, MEDIUM};
use crate::identity::identity_public_key::{KeyID, Purpose, SecurityLevel};

use crate::state_transition::state_transitions::document::documents_batch_transition::DocumentsBatchTransitionV0;
use crate::state_transition::StateTransitionIdentitySigned;

impl StateTransitionIdentitySigned for DocumentsBatchTransitionV0 {
    fn signature_public_key_id(&self) -> KeyID {
        self.signature_public_key_id
    }

    fn set_signature_public_key_id(&mut self, key_id: KeyID) {
        self.signature_public_key_id = key_id
    }

    fn security_level_requirement(&self, _purpose: Purpose) -> Vec<SecurityLevel> {
        // These are the available key levels that must sign the state transition
        // However the fact that it is signed by one of these does not guarantee that it
        // meets the security level requirement, as that is dictated from within the data
        // contract
        vec![CRITICAL, HIGH, MEDIUM]
    }
}
