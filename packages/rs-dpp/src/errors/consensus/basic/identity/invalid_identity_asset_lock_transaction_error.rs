use crate::errors::consensus::basic::BasicError;
use crate::errors::consensus::ConsensusError;
use crate::errors::ProtocolError;
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};
use thiserror::Error;

use bincode::{Decode, Encode};

#[derive(
    Error, Debug, Clone, Encode, Decode, PlatformSerialize, PlatformDeserialize, PartialEq,
)]
#[error("Invalid asset lock transaction: {message}")]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct InvalidIdentityAssetLockTransactionError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub message: String,
}

impl InvalidIdentityAssetLockTransactionError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl From<InvalidIdentityAssetLockTransactionError> for ConsensusError {
    fn from(err: InvalidIdentityAssetLockTransactionError) -> Self {
        Self::BasicError(BasicError::InvalidIdentityAssetLockTransactionError(err))
    }
}
