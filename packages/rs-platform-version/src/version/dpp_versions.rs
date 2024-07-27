use crate::version::{FeatureVersion, FeatureVersionBounds};

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DPPVersion {
    pub costs: CostVersions,
    pub validation: DPPValidationVersions,
    // TODO: Should be split by state transition type
    pub state_transition_serialization_versions: StateTransitionSerializationVersions,
    pub state_transition_conversion_versions: StateTransitionConversionVersions,
    pub state_transition_method_versions: StateTransitionMethodVersions,
    pub state_transitions: StateTransitionVersions,
    pub contract_versions: ContractVersions,
    pub document_versions: DocumentVersions,
    pub identity_versions: IdentityVersions,
    pub voting_versions: VotingVersions,
    pub asset_lock_versions: AssetLockVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct StateTransitionVersions {
    pub documents: DocumentTransitionVersions,
    pub identities: IdentityTransitionVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct IdentityTransitionVersions {
    pub max_public_keys_in_creation: u16,
    pub asset_locks: IdentityTransitionAssetLockVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct IdentityTransitionAssetLockVersions {
    pub required_asset_lock_duff_balance_for_processing_start_for_identity_create: u64,
    pub required_asset_lock_duff_balance_for_processing_start_for_identity_top_up: u64,
    pub validate_asset_lock_transaction_structure: FeatureVersion,
    pub validate_instant_asset_lock_proof_structure: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTransitionVersions {
    pub documents_batch_transition: DocumentsBatchTransitionVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentsBatchTransitionVersions {
    pub validation: DocumentsBatchTransitionValidationVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentsBatchTransitionValidationVersions {
    pub find_duplicates_by_id: FeatureVersion,
    pub validate_base_structure: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct CostVersions {
    pub signature_verify: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DPPValidationVersions {
    pub json_schema_validator: JsonSchemaValidatorVersions,
    pub data_contract: DataContractValidationVersions,
    pub document_type: DocumentTypeValidationVersions,
    pub voting: VotingValidationVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DataContractValidationVersions {
    pub validate: FeatureVersion,
    pub validate_config_update: FeatureVersion,
    pub validate_index_definitions: FeatureVersion,
    pub validate_index_naming_duplicates: FeatureVersion,
    pub validate_not_defined_properties: FeatureVersion,
    pub validate_property_definition: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct VotingValidationVersions {
    /// How long do we allow other contenders to join a contest after the first contender
    pub allow_other_contenders_time_ms: u64,
    /// How many votes do we allow from the same masternode?
    pub votes_allowed_per_masternode: u16,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTypeValidationVersions {
    pub validate_update: FeatureVersion,
    pub unique_index_limit: u16,
    pub contested_index_limit: u16,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct JsonSchemaValidatorVersions {
    pub new: FeatureVersion,
    pub validate: FeatureVersion,
    pub compile: FeatureVersion,
    pub compile_and_validate: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct StateTransitionMethodVersions {
    pub public_key_in_creation_methods: PublicKeyInCreationMethodVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct PublicKeyInCreationMethodVersions {
    pub from_public_key_signed_with_private_key: FeatureVersion,
    pub from_public_key_signed_external: FeatureVersion,
    pub hash: FeatureVersion,
    pub duplicated_key_ids_witness: FeatureVersion,
    pub duplicated_keys_witness: FeatureVersion,
    pub validate_identity_public_keys_structure: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct StateTransitionConversionVersions {
    pub identity_to_identity_create_transition: FeatureVersion,
    pub identity_to_identity_top_up_transition: FeatureVersion,
    pub identity_to_identity_withdrawal_transition: FeatureVersion,
    pub identity_to_identity_create_transition_with_signer: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct StateTransitionSerializationVersions {
    pub identity_public_key_in_creation: FeatureVersionBounds,
    pub identity_create_state_transition: FeatureVersionBounds,
    pub identity_update_state_transition: FeatureVersionBounds,
    pub identity_top_up_state_transition: FeatureVersionBounds,
    pub identity_credit_withdrawal_state_transition: FeatureVersionBounds,
    pub identity_credit_transfer_state_transition: FeatureVersionBounds,
    pub masternode_vote_state_transition: FeatureVersionBounds,
    pub contract_create_state_transition: FeatureVersionBounds,
    pub contract_update_state_transition: FeatureVersionBounds,
    pub documents_batch_state_transition: FeatureVersionBounds,
    pub document_base_state_transition: FeatureVersionBounds,
    pub document_create_state_transition: DocumentFeatureVersionBounds,
    pub document_replace_state_transition: DocumentFeatureVersionBounds,
    pub document_delete_state_transition: DocumentFeatureVersionBounds,
    pub document_transfer_state_transition: DocumentFeatureVersionBounds,
    pub document_update_price_state_transition: DocumentFeatureVersionBounds,
    pub document_purchase_state_transition: DocumentFeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentFeatureVersionBounds {
    pub bounds: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct ContractVersions {
    /// The maximum that we can store a data contract in the state. There is a possibility that a client
    /// sends a state transition serialized in a specific version and that the system re-serializes it
    /// to the current version, and in so doing increases it's size.
    pub max_serialized_size: u32,
    /// This is how we serialize and deserialize a contract
    pub contract_serialization_version: FeatureVersionBounds,
    /// This is the structure of the Contract as it is defined for code paths
    pub contract_structure_version: FeatureVersion,
    pub created_data_contract_structure: FeatureVersion,
    pub config: FeatureVersion,
    pub methods: DataContractMethodVersions,
    pub document_type_versions: DocumentTypeVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DataContractMethodVersions {
    pub validate_document: FeatureVersion,
    pub validate_update: FeatureVersion,
    pub schema: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTypeClassMethodVersions {
    pub try_from_schema: FeatureVersion,
    pub create_document_types_from_document_schemas: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTypeIndexVersions {
    pub index_levels_from_indices: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTypeVersions {
    pub index_versions: DocumentTypeIndexVersions,
    pub class_method_versions: DocumentTypeClassMethodVersions,
    /// This is for the overall structure of the document type, like DocumentTypeV0
    pub structure_version: FeatureVersion,
    pub schema: DocumentTypeSchemaVersions,
    pub methods: DocumentTypeMethodVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTypeMethodVersions {
    pub create_document_from_data: FeatureVersion,
    pub create_document_with_prevalidated_properties: FeatureVersion,
    pub prefunded_voting_balance_for_document: FeatureVersion,
    pub contested_vote_poll_for_document: FeatureVersion,
    pub estimated_size: FeatureVersion,
    pub index_for_types: FeatureVersion,
    pub max_size: FeatureVersion,
    pub serialize_value_for_key: FeatureVersion,
    pub deserialize_value_for_key: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentTypeSchemaVersions {
    pub enrich_with_base_schema: FeatureVersion,
    pub find_identifier_and_binary_paths: FeatureVersion,
    pub validate_max_depth: FeatureVersion,
    pub max_depth: u16,
    pub recursive_schema_validator_versions: RecursiveSchemaValidatorVersions,
    pub validate_schema_compatibility: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct RecursiveSchemaValidatorVersions {
    pub traversal_validator: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct AssetLockVersions {
    pub reduced_asset_lock_value: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct IdentityVersions {
    /// This is the structure of the Identity as it is defined for code paths
    pub identity_structure_version: FeatureVersion,
    pub identity_key_structure_version: FeatureVersion,
    pub identity_key_type_method_versions: IdentityKeyTypeMethodVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct VotingVersions {
    pub default_vote_poll_time_duration_ms: u64,
    pub contested_document_vote_poll_stored_info_version: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct IdentityKeyTypeMethodVersions {
    pub random_public_key_data: FeatureVersion,
    pub random_public_and_private_key_data: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentVersions {
    // This is for the overall structure of the document, like DocumentV0
    pub document_structure_version: FeatureVersion,
    pub document_serialization_version: FeatureVersionBounds,
    pub document_cbor_serialization_version: FeatureVersionBounds,
    pub extended_document_structure_version: FeatureVersion,
    pub extended_document_serialization_version: FeatureVersionBounds,
    pub document_method_versions: DocumentMethodVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DocumentMethodVersions {
    pub is_equal_ignoring_timestamps: FeatureVersion,
    pub hash: FeatureVersion,
    pub get_raw_for_contract: FeatureVersion,
    pub get_raw_for_document_type: FeatureVersion,
}
