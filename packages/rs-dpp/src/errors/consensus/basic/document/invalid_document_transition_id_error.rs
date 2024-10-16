use crate::consensus::basic::BasicError;
use crate::consensus::ConsensusError;
use crate::prelude::Identifier;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[error(
    "Invalid document transition id {}, expected {}",
    invalid_id,
    expected_id
)]
pub struct InvalidDocumentTransitionIdError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    expected_id: Identifier,
    invalid_id: Identifier,
}

impl InvalidDocumentTransitionIdError {
    pub fn new(expected_id: Identifier, invalid_id: Identifier) -> Self {
        Self {
            expected_id,
            invalid_id,
        }
    }

    pub fn expected_id(&self) -> Identifier {
        self.expected_id
    }

    pub fn invalid_id(&self) -> Identifier {
        self.invalid_id
    }
}

impl From<InvalidDocumentTransitionIdError> for ConsensusError {
    fn from(err: InvalidDocumentTransitionIdError) -> Self {
        Self::BasicError(BasicError::InvalidDocumentTransitionIdError(err))
    }
}
