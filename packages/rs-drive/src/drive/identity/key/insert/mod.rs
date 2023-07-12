mod insert_key_to_storage;
mod replace_key_in_storage;
mod insert_key_searchable_references;
mod insert_new_unique_key;
mod insert_new_non_unique_key;
mod create_key_tree_with_keys;
mod create_new_identity_key_query_trees;

use dpp::identity::IdentityPublicKey;

/// The contract apply info
pub enum ContractApplyInfo {
    /// Keys of the contract apply info
    Keys(Vec<IdentityPublicKey>),
}