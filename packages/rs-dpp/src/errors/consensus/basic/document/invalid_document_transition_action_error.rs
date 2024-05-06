use crate::errors::consensus::basic::BasicError;
use crate::errors::consensus::ConsensusError;
use crate::errors::ProtocolError;
use bincode::{Decode, Encode};
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};
use thiserror::Error;

#[derive(
    Error, Debug, Clone, PartialEq, Eq, Encode, Decode, PlatformSerialize, PlatformDeserialize,
)]
#[error("Document transition action {} is not supported", action)]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct InvalidDocumentTransitionActionError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub action: String,
}

impl InvalidDocumentTransitionActionError {
    pub fn new(action: String) -> Self {
        Self { action }
    }

    pub fn action(&self) -> &str {
        &self.action
    }
}

impl From<InvalidDocumentTransitionActionError> for ConsensusError {
    fn from(err: InvalidDocumentTransitionActionError) -> Self {
        Self::BasicError(BasicError::InvalidDocumentTransitionActionError(err))
    }
}
