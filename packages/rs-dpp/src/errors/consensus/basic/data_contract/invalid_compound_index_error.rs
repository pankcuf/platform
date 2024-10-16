use crate::consensus::basic::BasicError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consensus::ConsensusError;

use bincode::{Decode, Encode};

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[error("All or none of unique compound properties must be set for '{index_name}' index of '{document_type}' document")]
pub struct InvalidCompoundIndexError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    document_type: String,
    index_name: String,
}

impl InvalidCompoundIndexError {
    pub fn new(document_type: String, index_name: String) -> Self {
        Self {
            document_type,
            index_name,
        }
    }

    pub fn document_type(&self) -> &str {
        &self.document_type
    }
    pub fn index_name(&self) -> &str {
        &self.index_name
    }
}

impl From<InvalidCompoundIndexError> for ConsensusError {
    fn from(err: InvalidCompoundIndexError) -> Self {
        Self::BasicError(BasicError::InvalidCompoundIndexError(err))
    }
}
