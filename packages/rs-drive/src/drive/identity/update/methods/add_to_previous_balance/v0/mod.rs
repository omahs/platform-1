use grovedb::TransactionArg;
use dpp::fee::Credits;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::error::identity::IdentityError;
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    pub(super) fn add_to_previous_balance_v0(
        &self,
        identity_id: [u8; 32],
        previous_balance: Credits,
        added_balance: Credits,
        apply: bool,
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<AddToPreviousBalanceOutcome, Error> {
        if previous_balance == 0 {
            // Deduct debt from added amount if exists
            let negative_balance = self
                .fetch_identity_negative_balance_operations(
                    identity_id,
                    apply,
                    transaction,
                    drive_operations,
                    drive_version,
                )?
                .ok_or(Error::Drive(DriveError::CorruptedCodeExecution(
                    "there should always be a balance if apply is set to true",
                )))?;

            if apply {
                if negative_balance > added_balance {
                    Ok(AddToPreviousBalanceOutcome {
                        balance_modified: None,
                        negative_credit_balance_modified: Some(negative_balance - added_balance),
                    })
                } else {
                    let negative_credit_balance_modified =
                        if negative_balance > 0 { Some(0) } else { None };

                    Ok(AddToPreviousBalanceOutcome {
                        balance_modified: Some(added_balance - negative_balance),
                        negative_credit_balance_modified,
                    })
                }
            } else {
                // For dry run we want worst possible case + some room for tests (1000)
                Ok(AddToPreviousBalanceOutcome {
                    balance_modified: Some(MAX_CREDITS - 1000),
                    negative_credit_balance_modified: Some(0),
                })
            }
        } else {
            // Deduct added balance from existing one
            let new_balance =
                previous_balance
                    .checked_add(added_balance)
                    .ok_or(Error::Identity(IdentityError::CriticalBalanceOverflow(
                        "identity balance add overflow error",
                    )))?;

            Ok(AddToPreviousBalanceOutcome {
                balance_modified: Some(new_balance),
                negative_credit_balance_modified: None,
            })
        }
    }
}