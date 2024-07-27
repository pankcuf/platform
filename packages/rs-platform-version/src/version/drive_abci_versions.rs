use crate::version::{FeatureVersion, FeatureVersionBounds, OptionalFeatureVersion};

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciVersion {
    pub structs: DriveAbciStructureVersions,
    pub methods: DriveAbciMethodVersions,
    pub validation_and_processing: DriveAbciValidationVersions,
    pub query: DriveAbciQueryVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciQueryVersions {
    pub max_returned_elements: u16,
    pub response_metadata: FeatureVersion,
    pub proofs_query: FeatureVersionBounds,
    pub document_query: FeatureVersionBounds,
    pub prefunded_specialized_balances: DriveAbciQueryPrefundedSpecializedBalancesVersions,
    pub identity_based_queries: DriveAbciQueryIdentityVersions,
    pub data_contract_based_queries: DriveAbciQueryDataContractVersions,
    pub voting_based_queries: DriveAbciQueryVotingVersions,
    pub system: DriveAbciQuerySystemVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciQueryPrefundedSpecializedBalancesVersions {
    pub balance: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciQueryIdentityVersions {
    pub identity: FeatureVersionBounds,
    pub identities_contract_keys: FeatureVersionBounds,
    pub keys: FeatureVersionBounds,
    pub identity_nonce: FeatureVersionBounds,
    pub identity_contract_nonce: FeatureVersionBounds,
    pub balance: FeatureVersionBounds,
    pub balance_and_revision: FeatureVersionBounds,
    pub identity_by_public_key_hash: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
pub struct DriveAbciQueryVotingVersions {
    pub vote_polls_by_end_date_query: FeatureVersionBounds,
    pub contested_resource_vote_state: FeatureVersionBounds,
    pub contested_resource_voters_for_identity: FeatureVersionBounds,
    pub contested_resource_identity_vote_status: FeatureVersionBounds,
    pub contested_resources: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
pub struct DriveAbciQueryDataContractVersions {
    pub data_contract: FeatureVersionBounds,
    pub data_contract_history: FeatureVersionBounds,
    pub data_contracts: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciQuerySystemVersions {
    pub version_upgrade_state: FeatureVersionBounds,
    pub version_upgrade_vote_status: FeatureVersionBounds,
    pub epoch_infos: FeatureVersionBounds,
    pub path_elements: FeatureVersionBounds,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciStructureVersions {
    pub platform_state_structure: FeatureVersion,
    pub platform_state_for_saving_structure: FeatureVersion,
    pub state_transition_execution_context: FeatureVersion,
    pub commit: FeatureVersion,
    pub masternode: FeatureVersion,
    pub signature_verification_quorum_set: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciMethodVersions {
    pub engine: DriveAbciEngineMethodVersions,
    pub initialization: DriveAbciInitializationMethodVersions,
    pub core_based_updates: DriveAbciCoreBasedUpdatesMethodVersions,
    pub protocol_upgrade: DriveAbciProtocolUpgradeMethodVersions,
    pub block_fee_processing: DriveAbciBlockFeeProcessingMethodVersions,
    pub core_subsidy: DriveAbciCoreSubsidyMethodVersions,
    pub core_chain_lock: DriveAbciCoreChainLockMethodVersionsAndConstants,
    pub core_instant_send_lock: DriveAbciCoreInstantSendLockMethodVersions,
    pub fee_pool_inwards_distribution: DriveAbciFeePoolInwardsDistributionMethodVersions,
    pub fee_pool_outwards_distribution: DriveAbciFeePoolOutwardsDistributionMethodVersions,
    pub withdrawals: DriveAbciIdentityCreditWithdrawalMethodVersions,
    pub voting: DriveAbciVotingMethodVersions,
    pub state_transition_processing: DriveAbciStateTransitionProcessingMethodVersions,
    pub epoch: DriveAbciEpochMethodVersions,
    pub block_start: DriveAbciBlockStartMethodVersions,
    pub block_end: DriveAbciBlockEndMethodVersions,
    pub platform_state_storage: DriveAbciPlatformStateStorageMethodVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciValidationVersions {
    pub state_transitions: DriveAbciStateTransitionValidationVersions,
    pub process_state_transition: FeatureVersion,
    pub state_transition_to_execution_event_for_check_tx: FeatureVersion,
    pub penalties: PenaltyAmounts,
    pub event_constants: DriveAbciValidationConstants,
}

#[derive(Clone, Debug, Default)]
pub struct DriveAbciValidationConstants {
    pub maximum_vote_polls_to_process: u16,
    pub maximum_contenders_to_consider: u16,
}

/// All of these penalty amounts are in credits
#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct PenaltyAmounts {
    pub identity_id_not_correct: u64,
    pub unique_key_already_present: u64,
    pub validation_of_added_keys_structure_failure: u64,
    pub validation_of_added_keys_proof_of_possession_failure: u64,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciPlatformStateStorageMethodVersions {
    pub fetch_platform_state: FeatureVersion,
    pub store_platform_state: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciDocumentsStateTransitionValidationVersions {
    pub balance_pre_check: FeatureVersion,
    pub basic_structure: FeatureVersion,
    pub advanced_structure: FeatureVersion,
    pub revision: FeatureVersion,
    pub state: FeatureVersion,
    pub transform_into_action: FeatureVersion,
    pub data_triggers: DriveAbciValidationDataTriggerAndBindingVersions,
    pub document_create_transition_structure_validation: FeatureVersion,
    pub document_delete_transition_structure_validation: FeatureVersion,
    pub document_replace_transition_structure_validation: FeatureVersion,
    pub document_transfer_transition_structure_validation: FeatureVersion,
    pub document_purchase_transition_structure_validation: FeatureVersion,
    pub document_update_price_transition_structure_validation: FeatureVersion,
    pub document_create_transition_state_validation: FeatureVersion,
    pub document_delete_transition_state_validation: FeatureVersion,
    pub document_replace_transition_state_validation: FeatureVersion,
    pub document_transfer_transition_state_validation: FeatureVersion,
    pub document_purchase_transition_state_validation: FeatureVersion,
    pub document_update_price_transition_state_validation: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciValidationDataTriggerAndBindingVersions {
    pub bindings: FeatureVersion,
    pub triggers: DriveAbciValidationDataTriggerVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciValidationDataTriggerVersions {
    pub create_contact_request_data_trigger: FeatureVersion,
    pub create_domain_data_trigger: FeatureVersion,
    pub create_identity_data_trigger: FeatureVersion,
    pub create_feature_flag_data_trigger: FeatureVersion,
    pub create_masternode_reward_shares_data_trigger: FeatureVersion,
    pub delete_withdrawal_data_trigger: FeatureVersion,
    pub reject_data_trigger: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciStateTransitionValidationVersion {
    pub basic_structure: OptionalFeatureVersion,
    pub advanced_structure: OptionalFeatureVersion,
    pub identity_signatures: OptionalFeatureVersion,
    pub advanced_minimum_balance_pre_check: OptionalFeatureVersion,
    pub nonce: OptionalFeatureVersion,
    pub state: FeatureVersion,
    pub transform_into_action: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciStateTransitionValidationVersions {
    pub common_validation_methods: DriveAbciStateTransitionCommonValidationVersions,
    pub max_asset_lock_usage_attempts: u16,
    pub identity_create_state_transition: DriveAbciStateTransitionValidationVersion,
    pub identity_update_state_transition: DriveAbciStateTransitionValidationVersion,
    pub identity_top_up_state_transition: DriveAbciStateTransitionValidationVersion,
    pub identity_credit_withdrawal_state_transition: DriveAbciStateTransitionValidationVersion,
    pub identity_credit_transfer_state_transition: DriveAbciStateTransitionValidationVersion,
    pub masternode_vote_state_transition: DriveAbciStateTransitionValidationVersion,
    pub contract_create_state_transition: DriveAbciStateTransitionValidationVersion,
    pub contract_update_state_transition: DriveAbciStateTransitionValidationVersion,
    pub documents_batch_state_transition: DriveAbciDocumentsStateTransitionValidationVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciStateTransitionCommonValidationVersions {
    pub asset_locks: DriveAbciAssetLockValidationVersions,
    pub validate_identity_public_key_contract_bounds: FeatureVersion,
    pub validate_identity_public_key_ids_dont_exist_in_state: FeatureVersion,
    pub validate_identity_public_key_ids_exist_in_state: FeatureVersion,
    pub validate_state_transition_identity_signed: FeatureVersion,
    pub validate_unique_identity_public_key_hashes_in_state: FeatureVersion,
    pub validate_master_key_uniqueness: FeatureVersion,
    pub validate_simple_pre_check_balance: FeatureVersion,
}

#[derive(Clone, Copy, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciAssetLockValidationVersions {
    pub fetch_asset_lock_transaction_output_sync: FeatureVersion,
    pub verify_asset_lock_is_not_spent_and_has_enough_balance: FeatureVersion,
}

#[derive(Clone, Copy, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciEngineMethodVersions {
    pub init_chain: FeatureVersion,
    pub check_tx: FeatureVersion,
    pub run_block_proposal: FeatureVersion,
    pub finalize_block_proposal: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciCoreBasedUpdatesMethodVersions {
    pub update_core_info: FeatureVersion,
    pub update_masternode_list: FeatureVersion,
    pub update_quorum_info: FeatureVersion,
    pub masternode_updates: DriveAbciMasternodeIdentitiesUpdatesMethodVersions,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciMasternodeIdentitiesUpdatesMethodVersions {
    pub get_voter_identity_key: FeatureVersion,
    pub get_operator_identity_keys: FeatureVersion,
    pub get_owner_identity_key: FeatureVersion,
    pub get_voter_identifier_from_masternode_list_item: FeatureVersion,
    pub get_operator_identifier_from_masternode_list_item: FeatureVersion,
    pub create_operator_identity: FeatureVersion,
    pub create_owner_identity: FeatureVersion,
    pub create_voter_identity: FeatureVersion,
    pub disable_identity_keys: FeatureVersion,
    pub update_masternode_identities: FeatureVersion,
    pub update_operator_identity: FeatureVersion,
    pub update_owner_withdrawal_address: FeatureVersion,
    pub update_voter_identity: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciInitializationMethodVersions {
    pub initial_core_height_and_time: FeatureVersion,
    pub create_genesis_state: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciBlockFeeProcessingMethodVersions {
    pub add_process_epoch_change_operations: FeatureVersion,
    pub process_block_fees: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciCoreSubsidyMethodVersions {
    pub epoch_core_reward_credits_for_distribution: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciCoreInstantSendLockMethodVersions {
    pub verify_recent_signature_locally: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciCoreChainLockMethodVersionsAndConstants {
    pub choose_quorum: FeatureVersion,
    pub verify_chain_lock: FeatureVersion,
    pub verify_chain_lock_locally: FeatureVersion,
    pub verify_chain_lock_through_core: FeatureVersion,
    pub make_sure_core_is_synced_to_chain_lock: FeatureVersion,
    pub recent_block_count_amount: u32, //what constitutes a recent block, for v0 it's 2.
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciFeePoolInwardsDistributionMethodVersions {
    pub add_distribute_block_fees_into_pools_operations: FeatureVersion,
    pub add_distribute_storage_fee_to_epochs_operations: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciFeePoolOutwardsDistributionMethodVersions {
    pub add_distribute_fees_from_oldest_unpaid_epoch_pool_to_proposers_operations: FeatureVersion,
    pub add_epoch_pool_to_proposers_payout_operations: FeatureVersion,
    pub find_oldest_epoch_needing_payment: FeatureVersion,
    pub fetch_reward_shares_list_for_masternode: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciEpochMethodVersions {
    pub gather_epoch_info: FeatureVersion,
    pub get_genesis_time: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciBlockStartMethodVersions {
    pub clear_drive_block_cache: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciBlockEndMethodVersions {
    pub update_state_cache: FeatureVersion,
    pub update_drive_cache: FeatureVersion,
    pub validator_set_update: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciVotingMethodVersions {
    pub keep_record_of_finished_contested_resource_vote_poll: FeatureVersion,
    pub clean_up_after_vote_poll_end: FeatureVersion,
    pub clean_up_after_contested_resources_vote_poll_end: FeatureVersion,
    pub check_for_ended_vote_polls: FeatureVersion,
    pub tally_votes_for_contested_document_resource_vote_poll: FeatureVersion,
    pub award_document_to_winner: FeatureVersion,
    pub delay_vote_poll: FeatureVersion,
    pub run_dao_platform_events: FeatureVersion,
    pub remove_votes_for_removed_masternodes: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciIdentityCreditWithdrawalMethodVersions {
    pub build_untied_withdrawal_transactions_from_documents: FeatureVersion,
    pub dequeue_and_build_unsigned_withdrawal_transactions: FeatureVersion,
    pub fetch_transactions_block_inclusion_status: FeatureVersion,
    pub pool_withdrawals_into_transactions_queue: FeatureVersion,
    pub update_broadcasted_withdrawal_statuses: FeatureVersion,
    pub append_signatures_and_broadcast_withdrawal_transactions: FeatureVersion,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciProtocolUpgradeMethodVersions {
    pub check_for_desired_protocol_upgrade: FeatureVersion,
    pub upgrade_protocol_version_on_epoch_change: FeatureVersion,
    pub protocol_version_upgrade_percentage_needed: u64,
}

#[derive(Clone, Debug, Default)]
#[ferment_macro::export]
pub struct DriveAbciStateTransitionProcessingMethodVersions {
    pub execute_event: FeatureVersion,
    pub process_raw_state_transitions: FeatureVersion,
    pub decode_raw_state_transitions: FeatureVersion,
    pub validate_fees_of_event: FeatureVersion,
}
