mod v0;

use std::sync::Arc;
use grovedb::TransactionArg;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::contract::ContractFetchInfo;
use crate::drive::Drive;
use crate::error::drive::DriveError;
use crate::error::Error;

impl Drive {
    /// Returns the contract fetch info with the given ID if it's in cache.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - A 32-byte array representing the unique identifier of the contract.
    ///
    /// * `transaction` - A transaction that requests the contract.
    ///
    /// * `drive_version` - The version of the drive used to select the correct method version.
    ///
    /// # Returns
    ///
    /// * `Option<Arc<ContractFetchInfo>>` - An `Option` wrapping an `Arc` to the `ContractFetchInfo`.
    /// If a contract with the given ID exists in the cache, the function returns `Some(Arc<ContractFetchInfo>)`,
    /// otherwise it returns `None`.
    ///
    /// # Errors
    ///
    /// This function will return an `Error` if the drive version does not match any of the implemented method versions.
    pub fn get_cached_contract_with_fetch_info(
        &self,
        contract_id: [u8; 32],
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<Option<Arc<ContractFetchInfo>>, Error> {
        match drive_version.methods.contract.get.get_cached_contract_with_fetch_info {
            0 => Ok(self.get_cached_contract_with_fetch_info_v0(contract_id, transaction)),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "get_cached_contract_with_fetch_info".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}