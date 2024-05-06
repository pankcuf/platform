use std::collections::BTreeMap;

use platform_value::btreemap_extensions::BTreeValueRemoveFromMapHelper;
use platform_value::{IntegerReplacementType, ReplacementType, Value};

use crate::{data_contract::DataContract, errors::ProtocolError};

use platform_version::TryIntoPlatformVersioned;
use platform_version::version::PlatformVersion;
use crate::data_contract::conversion::value::v0::DataContractValueConversionMethodsV0;
use crate::state_transition::{StateTransitionFieldTypes, StateTransitionValueConvert};
use crate::state_transition::state_transitions::contract::data_contract_create_transition::{DataContractCreateTransitionV0};
use crate::state_transition::state_transitions::contract::data_contract_create_transition::fields::*;
use crate::state_transition::state_transitions::contract::data_contract_create_transition::fields::{BINARY_FIELDS, IDENTIFIER_FIELDS, U32_FIELDS};

impl<'a> StateTransitionValueConvert<'a> for DataContractCreateTransitionV0 {
    fn from_object(
        mut raw_object: Value,
        platform_version: &PlatformVersion,
    ) -> Result<Self, ProtocolError> {
        Ok(DataContractCreateTransitionV0 {
            signature: raw_object
                .remove_optional_binary_data(SIGNATURE)
                .map_err(ProtocolError::ValueError)?
                .unwrap_or_default(),
            signature_public_key_id: raw_object
                .get_optional_integer(SIGNATURE_PUBLIC_KEY_ID)
                .map_err(ProtocolError::ValueError)?
                .unwrap_or_default(),
            entropy: raw_object
                .remove_optional_bytes_32(ENTROPY)
                .map_err(ProtocolError::ValueError)?
                .unwrap_or_default(),
            data_contract: DataContract::from_value(
                raw_object.remove(DATA_CONTRACT).map_err(|_| {
                    ProtocolError::DecodingError(
                        "data contract missing on state transition".to_string(),
                    )
                })?,
                true,
                platform_version,
            )?
            .try_into_platform_versioned(platform_version)?,
        })
    }

    fn from_value_map(
        mut raw_value_map: BTreeMap<String, Value>,
        platform_version: &PlatformVersion,
    ) -> Result<Self, ProtocolError> {
        Ok(DataContractCreateTransitionV0 {
            signature: raw_value_map
                .remove_optional_binary_data(SIGNATURE)
                .map_err(ProtocolError::ValueError)?
                .unwrap_or_default(),
            signature_public_key_id: raw_value_map
                .remove_optional_integer(SIGNATURE_PUBLIC_KEY_ID)
                .map_err(ProtocolError::ValueError)?
                .unwrap_or_default(),
            entropy: raw_value_map
                .remove_optional_bytes_32(ENTROPY)
                .map_err(ProtocolError::ValueError)?
                .unwrap_or_default(),
            data_contract: DataContract::from_value(
                raw_value_map
                    .remove(DATA_CONTRACT)
                    .ok_or(ProtocolError::DecodingError(
                        "data contract missing on state transition".to_string(),
                    ))?,
                false,
                platform_version,
            )?
            .try_into_platform_versioned(platform_version)?,
        })
    }

    fn clean_value(value: &mut Value) -> Result<(), ProtocolError> {
        value.replace_at_paths(IDENTIFIER_FIELDS, ReplacementType::Identifier)?;
        value.replace_at_paths(BINARY_FIELDS, ReplacementType::BinaryBytes)?;
        value.replace_integer_type_at_paths(U32_FIELDS, IntegerReplacementType::U32)?;
        Ok(())
    }

    fn to_object(&self, skip_signature: bool) -> Result<Value, ProtocolError> {
        let mut object: Value = platform_value::to_value(self)?;
        if skip_signature {
            Self::signature_property_paths()
                .into_iter()
                .try_for_each(|path| {
                    object
                        .remove_values_matching_path(path)
                        .map_err(ProtocolError::ValueError)
                        .map(|_| ())
                })?;
        }
        Ok(object)
    }

    fn to_cleaned_object(&self, skip_signature: bool) -> Result<Value, ProtocolError> {
        let mut object: Value = platform_value::to_value(self)?;
        if skip_signature {
            Self::signature_property_paths()
                .into_iter()
                .try_for_each(|path| {
                    object
                        .remove_values_matching_path(path)
                        .map_err(ProtocolError::ValueError)
                        .map(|_| ())
                })?;
        }
        Ok(object)
    }
}
