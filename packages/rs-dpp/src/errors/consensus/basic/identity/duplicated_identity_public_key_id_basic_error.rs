use crate::errors::consensus::basic::BasicError;
use crate::errors::consensus::ConsensusError;
use crate::errors::ProtocolError;
use crate::identity::identity_public_key::KeyID;
use bincode::{Decode, Encode};
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};
use thiserror::Error;

#[derive(
    Error, Debug, Clone, PartialEq, Eq, Encode, Decode, PlatformSerialize, PlatformDeserialize,
)]
#[error("Duplicated public key ids {duplicated_ids:?} found")]
#[platform_serialize(unversioned)]
#[ferment_macro::export]
pub struct DuplicatedIdentityPublicKeyIdBasicError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub duplicated_ids: Vec<KeyID>,
}

impl DuplicatedIdentityPublicKeyIdBasicError {
    pub fn new(duplicated_ids: Vec<KeyID>) -> Self {
        Self { duplicated_ids }
    }

    pub fn duplicated_ids(&self) -> &Vec<KeyID> {
        &self.duplicated_ids
    }
}
impl From<DuplicatedIdentityPublicKeyIdBasicError> for ConsensusError {
    fn from(err: DuplicatedIdentityPublicKeyIdBasicError) -> Self {
        Self::BasicError(BasicError::DuplicatedIdentityPublicKeyIdBasicError(err))
    }
}
