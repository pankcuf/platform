use crate::consensus::basic::BasicError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consensus::ConsensusError;

use bincode::{Decode, Encode};

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[error("Invalid JSON Schema $ref: {message}")]
pub struct InvalidJsonSchemaRefError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    message: String,
}

impl InvalidJsonSchemaRefError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

impl From<InvalidJsonSchemaRefError> for ConsensusError {
    fn from(err: InvalidJsonSchemaRefError) -> Self {
        Self::BasicError(BasicError::InvalidJsonSchemaRefError(err))
    }
}
