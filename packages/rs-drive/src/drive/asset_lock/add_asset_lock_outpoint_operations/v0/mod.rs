use crate::drive::grove_operations::DirectQueryType::{StatefulDirectQuery, StatelessDirectQuery};
use crate::drive::grove_operations::QueryTarget::QueryTargetValue;
use crate::drive::object_size_info::PathKeyElementInfo::PathFixedSizeKeyRefElement;
use crate::drive::{Drive, RootTree};
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;
use dpp::platform_value::Bytes36;
use grovedb::batch::KeyInfoPath;
use grovedb::Element::Item;
use grovedb::{EstimatedLayerInformation, TransactionArg};
use std::collections::HashMap;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::asset_lock::asset_lock_storage_path;


impl Drive {

    /// Adds operations to a given `outpoint` if it is present in the estimated costs.
    ///
    /// # Arguments
    ///
    /// * `&self` - A reference to the current object.
    /// * `outpoint` - An `OutPoint` reference to be potentially modified.
    /// * `estimated_costs_only_with_layer_info` - A mutable reference to an optional `HashMap` that contains layer information.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of `LowLevelDriveOperation` if successful, or an `Error` otherwise.
    pub(super) fn add_asset_lock_outpoint_operations_v0(
        &self,
        outpoint: &Bytes36,
        estimated_costs_only_with_layer_info: &mut Option<
            HashMap<KeyInfoPath, EstimatedLayerInformation>,
        >,
        drive_version: &DriveVersion,
    ) -> Result<Vec<LowLevelDriveOperation>, Error> {
        let mut drive_operations = vec![];
        if let Some(estimated_costs_only_with_layer_info) = estimated_costs_only_with_layer_info {
            Self::add_estimation_costs_for_adding_asset_lock(estimated_costs_only_with_layer_info);
        }
        self.batch_insert(
            PathFixedSizeKeyRefElement((
                asset_lock_storage_path(),
                outpoint.as_slice(),
                Item(vec![], None),
            )),
            &mut drive_operations,
            drive_version,
        )?;
        Ok(drive_operations)
    }
}
