use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::prelude::Identifier;

pub mod calculate_operation_fees;
pub mod calculate_state_transition_fee_factory;
pub mod calculate_state_transition_fee_from_operations_factory;
pub mod constants;
pub mod operations;

pub type Credits = u64;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FeeResult {
    pub storage_fee: Credits,
    pub processing_fee: Credits,
    pub fee_refunds: Vec<Refunds>,
    pub total_refunds: Credits,
    pub desired_amount: Credits,
    pub required_amount: Credits,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DummyFeesResult {
    pub storage: Credits,
    pub processing: Credits,
    pub fee_refunds: Vec<Refunds>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "camelCase")]
pub struct Refunds {
    pub identifier: Identifier,
    pub credits_per_epoch: HashMap<String, Credits>,
}
