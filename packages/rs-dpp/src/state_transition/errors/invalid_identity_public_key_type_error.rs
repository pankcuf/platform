use thiserror::Error;

use crate::identity::identity_public_key::KeyType;
use crate::errors::ProtocolError;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Invalid signature type")]
#[ferment_macro::export]
pub struct InvalidIdentityPublicKeyTypeError {
    pub public_key_type: KeyType,
}

impl InvalidIdentityPublicKeyTypeError {
    pub fn new(public_key_type: KeyType) -> Self {
        Self { public_key_type }
    }

    pub fn public_key_type(&self) -> KeyType {
        self.public_key_type
    }
}

impl From<InvalidIdentityPublicKeyTypeError> for ProtocolError {
    fn from(err: InvalidIdentityPublicKeyTypeError) -> Self {
        Self::InvalidIdentityPublicKeyTypeError(err)
    }
}
