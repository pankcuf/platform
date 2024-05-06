use crate::errors::consensus::signature::signature_error::SignatureError;
use crate::errors::consensus::ConsensusError;
use crate::errors::ProtocolError;
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};
use thiserror::Error;

use bincode::{Decode, Encode};

#[derive(
    Error,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Default,
    Encode,
    Decode,
    PlatformSerialize,
    PlatformDeserialize,
)]
#[error("ecdsa signing error {message}")]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct BasicECDSAError {
    pub message: String,
}

/*

DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

*/

impl BasicECDSAError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<BasicECDSAError> for ConsensusError {
    fn from(err: BasicECDSAError) -> Self {
        Self::SignatureError(SignatureError::BasicECDSAError(err))
    }
}
