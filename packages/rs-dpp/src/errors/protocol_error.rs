use thiserror::Error as ThisError;

use crate::errors::consensus::basic::state_transition::InvalidStateTransitionTypeError;
use crate::errors::consensus::signature::{
    InvalidSignaturePublicKeySecurityLevelError, PublicKeyIsDisabledError,
};
use crate::errors::consensus::ConsensusError;
use crate::data_contract::errors::DataContractError;
use crate::data_contract::errors::DataContractNotPresentError;
use crate::data_contract::errors::IdentityNotPresentError;
use crate::data_contract::errors::InvalidDataContractError;
use crate::data_contract::errors::InvalidDocumentTypeError;
use crate::data_contract::errors::StructureError;

use crate::document::errors::DocumentError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::invalid_identity_public_key_type_error::InvalidIdentityPublicKeyTypeError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::invalid_signature_public_key_error::InvalidSignaturePublicKeyError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::public_key_mismatch_error::PublicKeyMismatchError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::public_key_security_level_not_met_error::PublicKeySecurityLevelNotMetError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::state_transition_error::StateTransitionError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::state_transition_is_not_signed_error::StateTransitionIsNotSignedError;
#[cfg(feature = "state-transition-validation")]
use crate::state_transition::errors::wrong_public_key_purpose_error::WrongPublicKeyPurposeError;

use crate::errors::non_consensus_error::NonConsensusError;
use crate::errors::compatible_protocol_version_is_not_defined_error::CompatibleProtocolVersionIsNotDefinedError;
use crate::errors::dpp_init_error::DashPlatformProtocolInitError;
use crate::errors::invalid_vector_size_error::InvalidVectorSizeError;
use crate::errors::serde_parsing_error::SerdeParsingError;

//use dashcore::consensus::encode::Error as DashCoreError;

use platform_version::version::FeatureVersion;
use platform_value::{Error, Value};
use platform_version::error::PlatformVersionError;

#[derive(ThisError, Debug)]
#[ferment_macro::export]
pub enum ProtocolError {
    #[error("Identifier Error: {0}")]
    IdentifierError(String),
    #[error("String Decode Error {0}")]
    StringDecodeError(String),
    #[error("Public key data is not set")]
    EmptyPublicKeyDataError,
    #[error("Payload reached a {max_size_kbytes}KB limit")]
    MaxEncodedBytesReachedError {
        max_size_kbytes: usize,
        size_hit: usize,
    },
    #[error("Encoding Error - {0}")]
    EncodingError(String),
    #[error("Decoding Error - {0}")]
    DecodingError(String),
    #[error("File not found Error - {0}")]
    FileNotFound(String),

    /// Platform expected some specific versions
    #[error(
    "dpp received not allowed version on {method}, allowed versions: {allowed_versions:?}, received: {received}"
    )]
    UnsupportedVersionMismatch {
        /// method
        method: String,
        /// the allowed versions for this method
        allowed_versions: Vec<FeatureVersion>,
        /// requested core height
        received: FeatureVersion,
    },

    /// Platform expected some specific versions
    #[error(
        "dpp unknown version on {method}, known versions: {known_versions:?}, received: {received}"
    )]
    UnknownVersionMismatch {
        /// method
        method: String,
        /// the allowed versions for this method
        known_versions: Vec<FeatureVersion>,
        /// requested core height
        received: FeatureVersion,
    },
    #[error("current platform version not initialized")]
    CurrentProtocolVersionNotInitialized,
    #[error("unknown version error {0}")]
    UnknownVersionError(String),
    #[error("unknown protocol version error {0}")]
    UnknownProtocolVersionError(String),
    #[error("Not included or invalid protocol version")]
    NoProtocolVersionError,
    #[error("Parsing error: {0}")]
    ParsingError(String),

    #[error(transparent)]
    ParsingJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    Error(#[from] anyhow::Error),

    #[error("Invalid key contract bounds error {0}")]
    InvalidKeyContractBoundsError(String),

    #[error("unknown storage key requirements {0}")]
    UnknownStorageKeyRequirements(String),

    #[error(transparent)]
    DataContractError(#[from] DataContractError),

    #[cfg(all(feature = "state-transitions", feature = "validation"))]
    #[error(transparent)]
    StateTransitionError(#[from] StateTransitionError),

    #[error(transparent)]
    StructureError(#[from] StructureError),

    #[error(transparent)]
    PlatformVersionError(#[from] PlatformVersionError),

    #[error(transparent)]
    ConsensusError(Box<ConsensusError>),

    #[error(transparent)]
    Document(Box<DocumentError>),

    #[error("Generic Error: {0}")]
    Generic(String),

    #[cfg(feature = "message-signing")]
    #[error("Invalid signing type error: {0}")]
    InvalidSigningKeyTypeError(String),

    // State Transition Errors
    #[cfg(any(
        feature = "state-transition-validation",
        feature = "state-transition-signing"
    ))]
    #[error(transparent)]
    InvalidIdentityPublicKeyTypeError(InvalidIdentityPublicKeyTypeError),
    #[cfg(feature = "state-transition-validation")]
    #[error(transparent)]
    StateTransitionIsNotSignedError(StateTransitionIsNotSignedError),
    #[cfg(feature = "state-transition-validation")]
    #[error(transparent)]
    PublicKeySecurityLevelNotMetError(PublicKeySecurityLevelNotMetError),
    #[cfg(feature = "state-transition-validation")]
    #[error(transparent)]
    WrongPublicKeyPurposeError(WrongPublicKeyPurposeError),
    #[cfg(feature = "state-transition-validation")]
    #[error(transparent)]
    PublicKeyMismatchError(PublicKeyMismatchError),
    #[cfg(feature = "state-transition-validation")]
    #[error(transparent)]
    InvalidSignaturePublicKeyError(InvalidSignaturePublicKeyError),

    #[error(transparent)]
    NonConsensusError(#[from] NonConsensusError),

    #[error(transparent)]
    CompatibleProtocolVersionIsNotDefinedError(#[from] CompatibleProtocolVersionIsNotDefinedError),

    // Data Contract
    #[error("Data Contract already exists")]
    DataContractAlreadyExistsError,

    #[error(transparent)]
    InvalidDataContractError(InvalidDataContractError),

    #[error(transparent)]
    InvalidDocumentTypeError(InvalidDocumentTypeError),

    #[error(transparent)]
    DataContractNotPresentError(DataContractNotPresentError),

    #[error(transparent)]
    InvalidSignaturePublicKeySecurityLevelError(InvalidSignaturePublicKeySecurityLevelError),

    #[error(transparent)]
    InvalidStateTransitionTypeError(InvalidStateTransitionTypeError),

    #[error(transparent)]
    PublicKeyIsDisabledError(PublicKeyIsDisabledError),

    #[error(transparent)]
    IdentityNotPresentError(IdentityNotPresentError),

    /// Error
    #[error("overflow error: {0}")]
    Overflow(&'static str),

    /// Error
    #[error("missing key: {0}")]
    DesiredKeyWithTypePurposeSecurityLevelMissing(String),

    /// Value error
    #[error("value error: {0}")]
    ValueError(#[from] Error),

    /// Value error
    #[error("platform serialization error: {0}")]
    PlatformSerializationError(String),

    /// Value error
    #[error("platform deserialization error: {0}")]
    PlatformDeserializationError(String),

    /// Dash core error
    #[error("dash core error: {0}")]
    DashCoreError(#[from] dashcore::consensus::encode::Error),

    #[error("Invalid Identity: {errors:?}")]
    InvalidIdentityError {
        errors: Vec<ConsensusError>,
        raw_identity: Value,
    },

    #[error("Public key generation error {0}")]
    PublicKeyGenerationError(String),

    #[error("corrupted code execution: {0}")]
    CorruptedCodeExecution(String),

    #[error("corrupted serialization: {0}")]
    CorruptedSerialization(String),

    #[error("critical corrupted credits code execution: {0}")]
    CriticalCorruptedCreditsCodeExecution(String),

    #[error(transparent)]
    InvalidVectorSizeError(InvalidVectorSizeError),
}

impl From<&str> for ProtocolError {
    fn from(v: &str) -> ProtocolError {
        ProtocolError::Generic(String::from(v))
    }
}

impl From<String> for ProtocolError {
    fn from(v: String) -> ProtocolError {
        Self::from(v.as_str())
    }
}

impl From<ConsensusError> for ProtocolError {
    fn from(e: ConsensusError) -> Self {
        ProtocolError::ConsensusError(Box::new(e))
    }
}

impl From<DocumentError> for ProtocolError {
    fn from(e: DocumentError) -> Self {
        ProtocolError::Document(Box::new(e))
    }
}

impl From<SerdeParsingError> for ProtocolError {
    fn from(e: SerdeParsingError) -> Self {
        ProtocolError::ParsingError(e.to_string())
    }
}

impl From<DashPlatformProtocolInitError> for ProtocolError {
    fn from(e: DashPlatformProtocolInitError) -> Self {
        ProtocolError::Generic(e.to_string())
    }
}

impl From<InvalidVectorSizeError> for ProtocolError {
    fn from(err: InvalidVectorSizeError) -> Self {
        Self::InvalidVectorSizeError(err)
    }
}

