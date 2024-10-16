use crate::consensus::state::state_error::StateError;
use crate::consensus::ConsensusError;
use bincode::{Decode, Encode};
use platform_value::Identifier;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[error("Provided document {document_id} owner ID {document_owner_id} mismatch with existing {existing_document_owner_id}")]
pub struct DocumentOwnerIdMismatchError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    document_id: Identifier,
    document_owner_id: Identifier,
    existing_document_owner_id: Identifier,
}

impl DocumentOwnerIdMismatchError {
    pub fn new(
        document_id: Identifier,
        document_owner_id: Identifier,
        existing_document_owner_id: Identifier,
    ) -> Self {
        Self {
            document_id,
            document_owner_id,
            existing_document_owner_id,
        }
    }

    pub fn document_id(&self) -> &Identifier {
        &self.document_id
    }

    pub fn document_owner_id(&self) -> &Identifier {
        &self.document_owner_id
    }

    pub fn existing_document_owner_id(&self) -> &Identifier {
        &self.existing_document_owner_id
    }
}

impl From<DocumentOwnerIdMismatchError> for ConsensusError {
    fn from(err: DocumentOwnerIdMismatchError) -> Self {
        Self::StateError(StateError::DocumentOwnerIdMismatchError(err))
    }
}
