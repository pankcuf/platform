pub mod v0;

pub use v0::*;

use crate::data_contract::DataContract;
use crate::identity::signer::Signer;
use crate::identity::{identity_public_key::KeyID, PartialIdentity};
use crate::prelude::IdentityNonce;
use crate::state_transition::state_transitions::contract::data_contract_create_transition::{
    DataContractCreateTransition, DataContractCreateTransitionV0,
};
use crate::state_transition::StateTransition;
use platform_version::version::FeatureVersion;
use crate::ProtocolError;
use platform_version::version::PlatformVersion;

impl DataContractCreateTransitionMethodsV0 for DataContractCreateTransition {
    fn new_from_data_contract<S: Signer>(
        data_contract: DataContract,
        identity_nonce: IdentityNonce,
        identity: &PartialIdentity,
        key_id: KeyID,
        signer: &S,
        platform_version: &PlatformVersion,
        feature_version: Option<FeatureVersion>,
    ) -> Result<StateTransition, ProtocolError> {
        match feature_version.unwrap_or(
            platform_version
                .dpp
                .state_transition_serialization_versions
                .contract_create_state_transition
                .default_current_version,
        ) {
            0 => DataContractCreateTransitionV0::new_from_data_contract(
                data_contract,
                identity_nonce,
                identity,
                key_id,
                signer,
                platform_version,
                feature_version,
            ),
            v => Err(ProtocolError::UnknownVersionError(format!(
                "Unknown DataContractCreateTransition version for new_from_data_contract {v}"
            ))),
        }
    }
}
