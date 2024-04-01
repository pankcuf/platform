use crate::version::fee::storage::FeeStorageVersion;

pub const FEE_STORAGE_VERSION1: FeeStorageVersion = FeeStorageVersion {
    storage_disk_usage_credit_per_byte: 27000,
    storage_processing_credit_per_byte: 400,
    storage_load_credit_per_byte: 400,
    non_storage_load_credit_per_byte: 30,
    storage_seek_cost: 4000,
};
