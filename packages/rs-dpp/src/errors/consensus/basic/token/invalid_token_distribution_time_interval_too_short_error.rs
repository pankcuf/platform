use crate::consensus::basic::BasicError;
use crate::consensus::ConsensusError;
use crate::prelude::TimestampMillisInterval;
use crate::ProtocolError;
use bincode::{Decode, Encode};
use platform_serialization_derive::{PlatformDeserialize, PlatformSerialize};
use thiserror::Error;

#[derive(
    Error, Debug, Clone, PartialEq, Eq, Encode, Decode, PlatformSerialize, PlatformDeserialize,
)]
#[error("TimeBasedDistribution interval is too short: {interval}. Minimum allowed is 3,600,000 ms (1 hour).")]
#[platform_serialize(unversioned)]
#[cfg_attr(feature = "apple", ferment_macro::export)]
pub struct InvalidTokenDistributionTimeIntervalTooShortError {
    pub interval: TimestampMillisInterval,
}

impl InvalidTokenDistributionTimeIntervalTooShortError {
    pub fn new(interval: TimestampMillisInterval) -> Self {
        Self { interval }
    }

    pub fn interval(&self) -> TimestampMillisInterval {
        self.interval
    }
}

impl From<InvalidTokenDistributionTimeIntervalTooShortError> for ConsensusError {
    fn from(err: InvalidTokenDistributionTimeIntervalTooShortError) -> Self {
        Self::BasicError(BasicError::InvalidTokenDistributionTimeIntervalTooShortError(err))
    }
}
