use crate::drive::batch::GroveDbOpBatch;
use crate::drive::grove_operations::BatchDeleteApplyType::StatefulBatchDelete;
use crate::drive::grove_operations::BatchInsertApplyType;
use crate::drive::object_size_info::PathKeyElementInfo;
use std::collections::BTreeMap;

use crate::drive::{Drive, RootTree};
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::error::Error::GroveDB;
use crate::fee::op::LowLevelDriveOperation;
use crate::query::QueryItem;
use dpp::util::deserializer::ProtocolVersion;
use grovedb::query_result_type::QueryResultType;
use grovedb::{Element, PathQuery, Query, TransactionArg};
use integer_encoding::VarInt;
use nohash_hasher::IntMap;
use std::ops::RangeFull;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::protocol_upgrade::versions_counter_path_vec;


impl Drive {
    /// Fetch versions by count for the upgrade window
    pub(super) fn fetch_versions_with_counter_v0(
        &self,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<IntMap<ProtocolVersion, u64>, Error> {
        let mut version_counter = IntMap::<ProtocolVersion, u64>::default();
        let path_query = PathQuery::new_unsized(
            versions_counter_path_vec(),
            Query::new_single_query_item(QueryItem::RangeFull(RangeFull)),
        );
        let (results, _) = self.grove_get_path_query(
            &path_query,
            transaction,
            QueryResultType::QueryKeyElementPairResultType,
            &mut vec![],
            drive_version,
        )?;
        for (version_bytes, _count_element) in results.to_key_elements() {
            let version = ProtocolVersion::decode_var(version_bytes.as_slice())
                .ok_or(Error::Drive(DriveError::CorruptedElementType(
                    "encoded value could not be decoded",
                )))
                .map(|(value, _)| value)?;
            let count = u64::decode_var(version_bytes.as_slice())
                .ok_or(Error::Drive(DriveError::CorruptedElementType(
                    "encoded value could not be decoded",
                )))
                .map(|(value, _)| value)?;
            version_counter.insert(version, count);
        }
        Ok(version_counter)
    }
}