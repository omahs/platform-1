use grovedb::TransactionArg;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    /// We add to the total platform system credits when:
    /// - we create an identity
    /// - we top up an identity
    /// - through the block reward
    pub(super) fn add_to_system_credits_v0(
        &self,
        amount: u64,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<(), Error> {
        let mut drive_operations = vec![];
        let batch_operations =
            self.add_to_system_credits_operations(amount, &mut None, transaction, drive_version)?;
        let grove_db_operations =
            LowLevelDriveOperation::grovedb_operations_batch(&batch_operations);
        self.grove_apply_batch_with_add_costs(
            grove_db_operations,
            false,
            transaction,
            &mut drive_operations,
            drive_version
        )
    }
}