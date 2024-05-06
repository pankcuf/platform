use crate::errors::consensus::state::state_error::StateError;
use crate::errors::consensus::ConsensusError;
use crate::errors::ProtocolError;
use bincode::{Decode, Encode};
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};
use platform_value::Identifier;
use thiserror::Error;

#[derive(
    Error, Debug, Clone, PartialEq, Eq, Encode, Decode, PlatformSerialize, PlatformDeserialize,
)]
#[error("Document {document_id} createdAt and updatedAt should not be equal")]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct DocumentTimestampsAreEqualError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub document_id: Identifier,
}

impl DocumentTimestampsAreEqualError {
    pub fn new(document_id: Identifier) -> Self {
        Self { document_id }
    }

    pub fn document_id(&self) -> &Identifier {
        &self.document_id
    }
}

impl From<DocumentTimestampsAreEqualError> for ConsensusError {
    fn from(err: DocumentTimestampsAreEqualError) -> Self {
        Self::StateError(StateError::DocumentTimestampsAreEqualError(err))
    }
}
