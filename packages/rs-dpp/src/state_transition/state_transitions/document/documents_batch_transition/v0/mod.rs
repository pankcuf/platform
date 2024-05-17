mod identity_signed;
#[cfg(feature = "state-transition-json-conversion")]
mod json_conversion;
mod state_transition_like;
mod types;
mod v0_methods;
#[cfg(feature = "state-transition-value-conversion")]
mod value_conversion;
mod version;

use crate::identity::identity_public_key::KeyID;

use crate::state_transition::state_transitions::document::documents_batch_transition::document_transition::DocumentTransition;
use crate::errors::ProtocolError;
use bincode::{Decode, Encode};
use platform_serialization_derive::PlatformSignable;

use crate::prelude::UserFeeIncrease;
use platform_value::{BinaryData, Identifier};
#[cfg(feature = "state-transition-serde-conversion")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Encode, Decode, PlatformSignable)]
#[cfg_attr(
    feature = "state-transition-serde-conversion",
    derive(Serialize, Deserialize)
)]
#[derive(Default)]
#[ferment_macro::export]
pub struct DocumentsBatchTransitionV0 {
    pub owner_id: Identifier,
    pub transitions: Vec<DocumentTransition>,
    pub user_fee_increase: UserFeeIncrease,
    #[platform_signable(exclude_from_sig_hash)]
    pub signature_public_key_id: KeyID,
    #[platform_signable(exclude_from_sig_hash)]
    pub signature: BinaryData,
}
