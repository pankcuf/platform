mod v0;

pub use v0::*;

use crate::data_contract::DataContract;
use crate::identity::signer::Signer;
use crate::identity::PartialIdentity;
use crate::identity::identity_public_key::KeyID;
use crate::state_transition::state_transitions::contract::data_contract_update_transition::{
    DataContractUpdateTransition, DataContractUpdateTransitionV0,
};
use crate::state_transition::StateTransition;
use platform_version::version::protocol_version::FeatureVersion;
use crate::errors::ProtocolError;

use crate::prelude::{IdentityNonce, UserFeeIncrease};
use platform_version::version::PlatformVersion;

impl DataContractUpdateTransitionMethodsV0 for DataContractUpdateTransition {
    fn new_from_data_contract<S: Signer>(
        data_contract: DataContract,
        identity: &PartialIdentity,
        key_id: KeyID,
        identity_contract_nonce: IdentityNonce,
        user_fee_increase: UserFeeIncrease,
        signer: &S,
        platform_version: &PlatformVersion,
        feature_version: Option<FeatureVersion>,
    ) -> Result<StateTransition, ProtocolError> {
        match feature_version.unwrap_or(
            platform_version
                .dpp
                .state_transition_serialization_versions
                .contract_update_state_transition
                .default_current_version,
        ) {
            0 => DataContractUpdateTransitionV0::new_from_data_contract(
                data_contract,
                identity,
                key_id,
                identity_contract_nonce,
                user_fee_increase,
                signer,
                platform_version,
                feature_version,
            ),
            v => Err(ProtocolError::UnknownVersionError(format!(
                "Unknown DataContractUpdateTransition version for new_from_data_contract {v}"
            ))),
        }
    }
}
