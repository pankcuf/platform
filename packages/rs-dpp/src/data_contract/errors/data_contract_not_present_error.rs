use platform_value::Identifier;
use thiserror::Error;

use crate::errors::ProtocolError;

// @append_only
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("Data Contract is not present")]
#[ferment_macro::export]
pub struct DataContractNotPresentError {
    pub data_contract_id: Identifier,
}

impl DataContractNotPresentError {
    pub fn new(data_contract_id: Identifier) -> Self {
        Self { data_contract_id }
    }

    pub fn data_contract_id(&self) -> Identifier {
        self.data_contract_id
    }
}

impl From<DataContractNotPresentError> for ProtocolError {
    fn from(err: DataContractNotPresentError) -> Self {
        Self::DataContractNotPresentError(err)
    }
}
