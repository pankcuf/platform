use crate::identity::state_transition::AssetLockProved;
use crate::identity::state_transition::asset_lock_proof::AssetLockProof;
use crate::state_transition::state_transitions::identity::identity_topup_transition::v0::IdentityTopUpTransitionV0;
use crate::ProtocolError;

impl AssetLockProved for IdentityTopUpTransitionV0 {
    fn set_asset_lock_proof(
        &mut self,
        asset_lock_proof: AssetLockProof,
    ) -> Result<(), ProtocolError> {
        self.asset_lock_proof = asset_lock_proof;

        Ok(())
    }

    fn asset_lock_proof(&self) -> &AssetLockProof {
        &self.asset_lock_proof
    }
}
