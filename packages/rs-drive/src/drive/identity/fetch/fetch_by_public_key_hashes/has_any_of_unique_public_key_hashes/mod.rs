mod v0;


use crate::drive::Drive;
use crate::error::{Error, drive::DriveError};
use crate::fee::op::LowLevelDriveOperation;
use dpp::version::drive_versions::DriveVersion;
use grovedb::TransactionArg;
use std::convert::TryInto;

impl Drive {
    /// Checks if any keys with given public key hashes already exist in the unique tree.
    ///
    /// This function leverages the versioning system to direct the fetch operation to the appropriate handler based on the `DriveVersion` provided.
    ///
    /// # Arguments
    ///
    /// * `public_key_hashes` - A vector of public key hashes to be checked.
    /// * `transaction` - Transaction arguments.
    /// * `drive_version` - A reference to the drive version.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of public key hashes that already exist, otherwise an `Error` if the operation fails or the version is not supported.
    pub fn has_any_of_unique_public_key_hashes(
        &self,
        public_key_hashes: Vec<[u8; 20]>,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<Vec<[u8; 20]>, Error> {
        match drive_version.methods.identity.fetch.public_key_hashes.has_any_of_unique_public_key_hashes {
            0 => self.has_any_of_unique_public_key_hashes_v0(public_key_hashes, transaction, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "has_any_of_unique_public_key_hashes".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }

    /// Executes operations for checking if any keys with given public key hashes already exist in the unique tree.
    ///
    /// This function leverages the versioning system to direct the fetch operation to the appropriate handler based on the `DriveVersion` provided.
    ///
    /// # Arguments
    ///
    /// * `public_key_hashes` - A vector of public key hashes to be checked.
    /// * `transaction` - Transaction arguments.
    /// * `drive_operations` - A mutable reference to a vector of drive operations.
    /// * `drive_version` - A reference to the drive version.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of public key hashes that already exist, otherwise an `Error` if the operation fails or the version is not supported.
    pub(crate) fn has_any_of_unique_public_key_hashes_operations(
        &self,
        public_key_hashes: Vec<[u8; 20]>,
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<Vec<[u8; 20]>, Error> {
        match drive_version.methods.identity.fetch.public_key_hashes.has_any_of_unique_public_key_hashes {
            0 => self.has_any_of_unique_public_key_hashes_operations_v0(public_key_hashes, transaction, drive_operations, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "has_any_of_unique_public_key_hashes_operations".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}