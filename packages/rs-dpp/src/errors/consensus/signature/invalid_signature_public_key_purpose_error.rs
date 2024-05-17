use thiserror::Error;

use crate::errors::consensus::signature::signature_error::SignatureError;
use crate::errors::consensus::ConsensusError;
use crate::identity::identity_public_key::Purpose;

use crate::errors::ProtocolError;
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};

use bincode::{Decode, Encode};

#[derive(
    Error, Debug, Clone, PartialEq, Eq, Encode, Decode, PlatformSerialize, PlatformDeserialize,
)]
#[error("Invalid public key purpose {public_key_purpose}. The state transition requires {allowed_key_purpose}")]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct InvalidSignaturePublicKeyPurposeError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub public_key_purpose: Purpose,
    pub allowed_key_purpose: Purpose,
}

impl InvalidSignaturePublicKeyPurposeError {
    pub fn new(public_key_purpose: Purpose, allowed_key_purpose: Purpose) -> Self {
        Self {
            public_key_purpose,
            allowed_key_purpose,
        }
    }

    pub fn public_key_purpose(&self) -> Purpose {
        self.public_key_purpose
    }
    pub fn allowed_key_purpose(&self) -> Purpose {
        self.allowed_key_purpose
    }
}

impl From<InvalidSignaturePublicKeyPurposeError> for ConsensusError {
    fn from(err: InvalidSignaturePublicKeyPurposeError) -> Self {
        Self::SignatureError(SignatureError::InvalidSignaturePublicKeyPurposeError(err))
    }
}
