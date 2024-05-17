use platform_version::version::PlatformVersion;
use crate::errors::ProtocolError;
// use serde_json::Value as JsonValue;

pub trait DataContractJsonConversionMethodsV0 {
    fn from_json(
        json_value: serde_json::Value,
        validate: bool,
        platform_version: &PlatformVersion,
    ) -> Result<Self, ProtocolError>
    where
        Self: Sized;

    /// Returns Data Contract as a JSON Value
    fn to_json(&self, platform_version: &PlatformVersion) -> Result<serde_json::Value, ProtocolError>;
    /// Returns Data Contract as a JSON Value
    fn to_validating_json(
        &self,
        platform_version: &PlatformVersion,
    ) -> Result<serde_json::Value, ProtocolError>;
}
