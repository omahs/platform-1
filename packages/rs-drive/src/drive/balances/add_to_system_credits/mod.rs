mod v0;

use grovedb::TransactionArg;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::error::Error;
use crate::error::drive::DriveError;

impl Drive {
    /// Adds to the total platform system credits when:
    /// - we create an identity
    /// - we top up an identity
    /// - through the block reward
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount of system credits to be added.
    /// * `transaction` - A `TransactionArg` object representing the transaction to be used for adding to the system credits.
    /// * `drive_version` - A `DriveVersion` object specifying the version of the Drive.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - If successful, returns `Ok(())`. If an error occurs during the operation, returns an `Error`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the version of the Drive is unknown.
    pub fn add_to_system_credits(
        &self,
        amount: u64,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<(), Error> {
        match drive_version.methods.balances.add_to_system_credits {
            0 => self.add_to_system_credits_v0(amount, transaction, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "add_to_system_credits".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}