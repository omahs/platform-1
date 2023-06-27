mod v0;

use std::collections::HashMap;
use grovedb::batch::KeyInfoPath;
use grovedb::{EstimatedLayerInformation, TransactionArg};
use dpp::block::epoch::Epoch;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::contract::Contract;
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    /// Prepares the operations for deleting a document.
    ///
    /// # Parameters
    /// * `document_id`: The ID of the document to delete.
    /// * `contract`: The contract that contains the document.
    /// * `document_type_name`: The name of the document type.
    /// * `previous_batch_operations`: Previous batch operations to include.
    /// * `estimated_costs_only_with_layer_info`: Estimated costs with layer info.
    /// * `transaction`: The transaction argument.
    /// * `drive_version`: The drive version to select the correct function version to run.
    ///
    /// # Returns
    /// * `Ok(Vec<LowLevelDriveOperation>)` if the operation was successful.
    /// * `Err(DriveError::UnknownVersionMismatch)` if the drive version does not match known versions.
    pub(super) fn delete_document_for_contract_with_named_type_operations(
        &self,
        document_id: [u8; 32],
        contract: &Contract,
        document_type_name: &str,
        previous_batch_operations: Option<&mut Vec<LowLevelDriveOperation>>,
        estimated_costs_only_with_layer_info: &mut Option<
            HashMap<KeyInfoPath, EstimatedLayerInformation>,
        >,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<Vec<LowLevelDriveOperation>, Error> {
        match drive_version.methods.document.delete.delete_document_for_contract_with_named_type_operations {
            0 => {
                self.delete_document_for_contract_with_named_type_operations_v0(
                    document_id,
                    contract,
                    document_type_name,
                    previous_batch_operations,
                    estimated_costs_only_with_layer_info,
                    transaction,
                    drive_version,
                )
            },
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "delete_document_for_contract_with_named_type_operations".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}