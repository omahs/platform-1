mod v0;

use std::collections::BTreeMap;
use std::sync::Arc;
use grovedb::TransactionArg;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::contract::ContractFetchInfo;
use crate::drive::Drive;
use crate::error::drive::DriveError;
use crate::error::Error;

impl Drive {
    /// Retrieves the specified contracts and their associated fetch information.
    ///
    /// This method uses the `DriveVersion` to determine the correct version of the
    /// `get_contract_with_fetch_info` method to call for each contract. If the fetching process
    /// is successful, it returns a `BTreeMap` where each key-value pair represents a contract ID
    /// and its corresponding `ContractFetchInfo`. If there's any error during the process, it will return an `Error`.
    ///
    /// # Arguments
    ///
    /// * `contract_ids` - A slice of contract IDs. Each ID is a 32-byte array. The contract IDs are used to
    ///   fetch the corresponding contracts.
    /// * `add_to_cache_if_pulled` - A boolean flag that determines whether the fetched contracts should be
    ///   added to the cache if they were pulled from the storage.
    /// * `transaction` - A `TransactionArg` object which represents the transaction to be used for fetching the contracts.
    /// * `drive_version` - The version of the drive. It is used to select the appropriate version of the
    ///   `get_contract_with_fetch_info` method.
    ///
    /// # Returns
    ///
    /// * `Result<BTreeMap<[u8; 32], Option<Arc<ContractFetchInfo>>>, Error>` - If successful,
    ///   returns a `BTreeMap` where the keys are the contract IDs and the values are `Option`s
    ///   containing `Arc`s to the corresponding `ContractFetchInfo`. If an error occurs during the contract fetching,
    ///   an `Error` is returned.
    ///
    /// # Errors
    ///
    /// This function may return an `Error` if there's a failure in the contract fetching process or if the
    /// drive version does not match any of the implemented method versions.
    pub fn get_contracts_with_fetch_info(
        &self,
        contract_ids: &[[u8; 32]],
        add_to_cache_if_pulled: bool,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<BTreeMap<[u8; 32], Option<Arc<ContractFetchInfo>>>, Error> {
        match drive_version.methods.contract.get.get_contracts_with_fetch_info {
            0 => contract_ids
                .iter()
                .map(|contract_id| {
                    Ok((
                        *contract_id,
                        self.get_contracts_with_fetch_info_v0(
                            contract_ids,
                            add_to_cache_if_pulled,
                            transaction,
                            drive_version,
                        )?,
                    ))
                })
                .collect(),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "get_contracts_with_fetch_info".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}