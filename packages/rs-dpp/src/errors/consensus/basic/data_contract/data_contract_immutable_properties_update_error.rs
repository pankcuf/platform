use crate::consensus::basic::BasicError;
use platform_value::Value;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consensus::ConsensusError;

use bincode::{Decode, Encode};

#[derive(Error, Debug, Clone, Serialize, Deserialize, Encode, Decode)]
#[error("only $defs, version and documents fields are allowed to be updated. Forbidden operation '{operation}' on '{field_path}'")]
pub struct DataContractImmutablePropertiesUpdateError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    operation: String,
    field_path: String,
    old_value: Value,
    new_value: Value,
}

impl DataContractImmutablePropertiesUpdateError {
    pub fn new(operation: String, field_path: String, old_value: Value, new_value: Value) -> Self {
        Self {
            operation,
            field_path,
            old_value,
            new_value,
        }
    }

    pub fn operation(&self) -> String {
        self.operation.clone()
    }

    pub fn field_path(&self) -> String {
        self.field_path.clone()
    }

    pub fn old_value(&self) -> Value {
        self.old_value.clone()
    }

    pub fn new_value(&self) -> Value {
        self.new_value.clone()
    }
}

impl From<DataContractImmutablePropertiesUpdateError> for ConsensusError {
    fn from(err: DataContractImmutablePropertiesUpdateError) -> Self {
        Self::BasicError(BasicError::DataContractImmutablePropertiesUpdateError(err))
    }
}
