mod v0;

use std::collections::HashMap;
use grovedb::batch::KeyInfoPath;
use grovedb::{EstimatedLayerInformation, TransactionArg};
use grovedb::EstimatedLayerCount::{ApproximateElements, PotentiallyAtMaxElements};
use grovedb::EstimatedLayerSizes::AllSubtrees;
use grovedb::EstimatedSumTrees::NoSumTrees;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::defaults::DEFAULT_HASH_SIZE_U8;
use crate::drive::Drive;
use crate::drive::flags::StorageFlags;
use crate::drive::grove_operations::BatchInsertTreeApplyType;
use crate::drive::object_size_info::{DocumentAndContractInfo, PathInfo};
use crate::drive::object_size_info::DriveKeyInfo::KeyRef;
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::error::fee::FeeError;
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    /// Adds indices for an index level and recurses.
    pub(crate) fn add_indices_for_index_level_for_contract_operations(
        &self,
        document_and_contract_info: &DocumentAndContractInfo,
        index_path_info: PathInfo<0>,
        index_level: &IndexLevel,
        mut any_fields_null: bool,
        previous_batch_operations: &mut Option<&mut Vec<LowLevelDriveOperation>>,
        storage_flags: &Option<&StorageFlags>,
        estimated_costs_only_with_layer_info: &mut Option<
            HashMap<KeyInfoPath, EstimatedLayerInformation>,
        >,
        event_id: [u8; 32],
        transaction: TransactionArg,
        batch_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<(), Error> {
        match drive_version.methods.document.insert.add_indices_for_index_level_for_contract_operations {
            0 => {
                self.add_indices_for_index_level_for_contract_operations_v0(
                    document_and_contract_info,
                    index_path_info,
                    index_level,
                    any_fields_null,
                    previous_batch_operations,
                    storage_flags,
                    estimated_costs_only_with_layer_info,
                    event_id,
                    transaction,
                    batch_operations,
                    drive_version,
                )
            },
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "add_indices_for_index_level_for_contract_operations".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}