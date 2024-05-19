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
#[error("{document_id} document not for sale")]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct DocumentNotForSaleError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub document_id: Identifier,
}

impl DocumentNotForSaleError {
    pub fn new(document_id: Identifier) -> Self {
        Self { document_id }
    }

    pub fn document_id(&self) -> &Identifier {
        &self.document_id
    }
}

impl From<DocumentNotForSaleError> for ConsensusError {
    fn from(err: DocumentNotForSaleError) -> Self {
        Self::StateError(StateError::DocumentNotForSaleError(err))
    }
}
