use grovedb::{Element, TransactionArg};
use integer_encoding::VarInt;
use dpp::util::deserializer::ProtocolVersion;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::drive::grove_operations::BatchInsertApplyType;
use crate::drive::object_size_info::PathKeyElementInfo;
use crate::drive::system::misc_path;
use crate::drive::system::misc_tree_constants::NEXT_PROTOCOL_VERSION_STORAGE_KEY;
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    /// Gets the next protocol version from the backing store
    pub(super) fn fetch_next_protocol_version_v0(
        &self,
        transaction: TransactionArg,
    ) -> Result<Option<ProtocolVersion>, Error> {
        let misc_path = misc_path();
        self.grove.get_raw_optional((&misc_path).into(), NEXT_PROTOCOL_VERSION_STORAGE_KEY, transaction).unwrap().map_err(Error::GroveDB)
            .map(|maybe_element| {
                maybe_element.map(|e| {
                    let bytes = e.as_item_bytes()?;
                    let Some((protocol_version, _)) = ProtocolVersion::decode_var(bytes) else {
                        return Err(Error::Drive(DriveError::CorruptedSerialization("protocol version incorrectly serialized")))
                    };
                    Ok(protocol_version)
                }).transpose()
            })?
    }
}