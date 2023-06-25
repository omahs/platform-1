mod v0;

use std::collections::HashMap;
use grovedb::batch::{BatchApplyOptions, KeyInfoPath};
use grovedb::{EstimatedLayerInformation, GroveDb};
use grovedb::batch::estimated_costs::EstimatedCostsType::AverageCaseCostsType;
use crate::drive::batch::GroveDbOpBatch;
use crate::drive::Drive;
use crate::drive::grove_operations::push_drive_operation_result;
use crate::error::Error;
use crate::error::drive::DriveError;
use crate::fee::op::LowLevelDriveOperation;
use crate::query::GroveError;
use dpp::version::drive_versions::DriveVersion;

impl Drive {
    /// Retrieves the costs for the given batch of groveDB operations.
    /// The costs are then added to `drive_operations` for later processing.
    ///
    /// # Parameters
    /// * `ops`: The batch of groveDB operations to retrieve costs for.
    /// * `estimated_layer_info`: A map with estimated layer information.
    /// * `validate`: Specifies whether to validate that insertions do not override existing entries.
    /// * `drive_operations`: A vector to collect the costs of operations for later computation.
    /// * `drive_version`: The drive version to select the correct function version to run.
    ///
    /// # Returns
    /// * `Ok(())` if the operation was successful.
    /// * `Err(DriveError::UnknownVersionMismatch)` if the drive version does not match known versions.
    pub fn grove_batch_operations_costs(
        &self,
        ops: GroveDbOpBatch,
        estimated_layer_info: HashMap<KeyInfoPath, EstimatedLayerInformation>,
        validate: bool,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<(), Error> {
        match drive_version.grove_methods.costs.grove_batch_operations_costs {
            0 => self.grove_batch_operations_costs_v0(ops, estimated_layer_info, validate, drive_operations),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "grove_batch_operations_costs".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}