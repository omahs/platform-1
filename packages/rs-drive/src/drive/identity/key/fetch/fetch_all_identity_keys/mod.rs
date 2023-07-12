mod v0;

use std::collections::BTreeMap;
use grovedb::TransactionArg;
use dpp::identity::{IdentityPublicKey, KeyID};
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::drive::identity::key::fetch::IdentityKeysRequest;
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    /// Fetches all the keys of every kind for a specific Identity.
    ///
    /// This function uses the versioning system to call the appropriate handler based on the provided `DriveVersion`.
    ///
    /// # Arguments
    ///
    /// * `identity_id` - The identity id for which to fetch the keys.
    /// * `transaction` - Transaction arguments.
    /// * `drive_version` - A reference to the drive version.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a map of `KeyID` to `IdentityPublicKey`, otherwise an `Error` if the operation fails or the version is not supported.
    pub fn fetch_all_identity_keys(
        &self,
        identity_id: [u8; 32],
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<BTreeMap<KeyID, IdentityPublicKey>, Error> {
        match drive_version.methods.identity.keys.fetch.fetch_all_identity_keys {
            0 => self.fetch_all_identity_keys_v0(identity_id, transaction, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "fetch_all_identity_keys".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }

    /// Fetches the operations for fetching all the keys of every kind for a specific Identity.
    ///
    /// This function uses the versioning system to call the appropriate handler based on the provided `DriveVersion`.
    ///
    /// # Arguments
    ///
    /// * `identity_id` - The identity id for which to fetch the keys.
    /// * `transaction` - Transaction arguments.
    /// * `drive_operations` - A mutable reference to a vector of low level drive operations.
    /// * `drive_version` - A reference to the drive version.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a map of `KeyID` to `IdentityPublicKey` representing the fetched keys, otherwise an `Error` if the operation fails or the version is not supported.
    pub(crate) fn fetch_all_identity_keys_operations(
        &self,
        identity_id: [u8; 32],
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<BTreeMap<KeyID, IdentityPublicKey>, Error> {
        match drive_version.methods.identity.keys.fetch.fetch_all_identity_keys {
            0 => self.fetch_all_identity_keys_operations_v0(identity_id, transaction, drive_operations, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "fetch_all_identity_keys_operations".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}