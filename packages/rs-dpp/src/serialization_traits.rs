use crate::identity::KeyType;

use crate::validation::SimpleConsensusValidationResult;
use crate::{BlsModule, ProtocolError};
use platform_value::Value;

pub trait Signable {
    fn signable_bytes(&self) -> Result<Vec<u8>, ProtocolError>;
}

pub trait PlatformSerializable {
    fn serialize(&self) -> Result<Vec<u8>, ProtocolError>;

    /// If the trait is not used just do a simple serialize
    fn serialize_consume(self) -> Result<Vec<u8>, ProtocolError>
    where
        Self: Sized,
    {
        self.serialize()
    }
}

pub trait PlatformDeserializable {
    fn deserialize(data: &[u8]) -> Result<Self, ProtocolError>
    where
        Self: Sized;
}

pub trait ValueConvertible {
    fn to_object(&self) -> Result<Value, ProtocolError>;

    fn into_object(self) -> Result<Value, ProtocolError>;

    fn from_object(value: Value) -> Result<Self, ProtocolError>
    where
        Self: Sized;

    fn from_object_ref(value: &Value) -> Result<Self, ProtocolError>
    where
        Self: Sized;
}

pub trait PlatformMessageSignable {
    fn verify_signature(
        &self,
        public_key_type: KeyType,
        public_key_data: &[u8],
        signature: &[u8],
    ) -> Result<SimpleConsensusValidationResult, ProtocolError>;
    fn sign_by_private_key(
        &self,
        private_key: &[u8],
        key_type: KeyType,
        bls: &impl BlsModule,
    ) -> Result<Vec<u8>, ProtocolError>;
}
