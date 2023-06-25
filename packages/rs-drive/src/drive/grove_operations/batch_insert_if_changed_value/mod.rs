mod v0;

use grovedb::{Element, GroveDb, TransactionArg};
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::drive::flags::StorageFlags;
use crate::drive::grove_operations::{BatchInsertApplyType, BatchInsertTreeApplyType};
use crate::drive::object_size_info::{PathKeyElementInfo, PathKeyInfo};
use crate::drive::object_size_info::PathKeyElementInfo::{PathFixedSizeKeyRefElement, PathKeyElement, PathKeyElementSize, PathKeyRefElement, PathKeyUnknownElementSize};
use crate::drive::object_size_info::PathKeyInfo::{PathFixedSizeKey, PathFixedSizeKeyRef, PathKey, PathKeyRef, PathKeySize};
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;
use crate::fee::op::LowLevelDriveOperation::{CalculatedCostOperation, GroveOperation};

impl Drive {
    /// Pushes an "insert element if element was changed or is new" operation to `drive_operations`.
    /// Returns true if the path key already exists without references.
    ///
    /// # Parameters
    /// * `path_key_element_info`: Information about the path, key and element.
    /// * `apply_type`: The apply type for the operation.
    /// * `transaction`: The transaction argument for the operation.
    /// * `drive_operations`: The list of drive operations to append to.
    /// * `drive_version`: The drive version to select the correct function version to run.
    ///
    /// # Returns
    /// * `Ok((bool, Option<Element>))` if the operation was successful. Returns true if the path key already exists without references.
    /// * `Err(DriveError::UnknownVersionMismatch)` if the drive version does not match known versions.
    /// * `Err(DriveError::CorruptedCodeExecution)` if the operation is not supported.
    pub fn batch_insert_if_changed_value<const N: usize>(
        &self,
        path_key_element_info: PathKeyElementInfo<N>,
        apply_type: BatchInsertApplyType,
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        drive_version: &DriveVersion,
    ) -> Result<(bool, Option<Element>), Error> {
        match drive_version.grove_methods.batch.batch_insert_if_changed_value {
            0 => self.batch_insert_if_changed_value_v0(path_key_element_info, apply_type, transaction, drive_operations, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "batch_insert_if_changed_value".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}