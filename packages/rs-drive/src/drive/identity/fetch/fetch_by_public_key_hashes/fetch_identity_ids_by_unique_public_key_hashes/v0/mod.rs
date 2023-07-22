use crate::drive::grove_operations::DirectQueryType::StatefulDirectQuery;
use crate::drive::{
    non_unique_key_hashes_sub_tree_path, non_unique_key_hashes_sub_tree_path_vec,
    non_unique_key_hashes_tree_path, unique_key_hashes_tree_path, unique_key_hashes_tree_path_vec,
    Drive,
};
use crate::error::drive::DriveError;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;
use crate::query::{QueryItem, QueryResultEncoding};
use dpp::identity::Identity;
use dpp::platform_value::Value;
use dpp::Convertible;
use grovedb::query_result_type::QueryResultType;

use dpp::version::drive_versions::DriveVersion;
use grovedb::Element::Item;
use grovedb::{PathQuery, Query, SizedQuery, TransactionArg};
use std::collections::BTreeMap;
use std::ops::RangeFull;
use dpp::version::PlatformVersion;

impl Drive {
    /// Fetches identity ids with all its information from storage.
    pub(super) fn fetch_identity_ids_by_unique_public_key_hashes_v0(
        &self,
        public_key_hashes: &[[u8; 20]],
        transaction: TransactionArg,
        platform_version: &PlatformVersion,
    ) -> Result<BTreeMap<[u8; 20], Option<[u8; 32]>>, Error> {
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        self.fetch_identity_ids_by_unique_public_key_hashes_operations_v0(
            public_key_hashes,
            transaction,
            &mut drive_operations,
            platform_version,
        )
    }

    /// Given public key hashes, fetches identity ids from storage.
    pub(super) fn fetch_identity_ids_by_unique_public_key_hashes_operations_v0(
        &self,
        public_key_hashes: &[[u8; 20]],
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        platform_version: &PlatformVersion,
    ) -> Result<BTreeMap<[u8; 20], Option<[u8; 32]>>, Error> {
        let unique_key_hashes = unique_key_hashes_tree_path_vec();
        let mut query = Query::new();
        query.insert_keys(
            public_key_hashes
                .iter()
                .map(|key_hash| key_hash.to_vec())
                .collect(),
        );
        let sized_query = SizedQuery {
            query,
            limit: Some(public_key_hashes.len() as u16),
            offset: None,
        };
        let path_query = PathQuery::new(unique_key_hashes, sized_query);
        self.grove_get_raw_path_query_with_optional(
            &path_query,
            transaction,
            drive_operations,
            &platform_version.drive,
        )?
        .into_iter()
        .map(|(_, key, element)| {
            let identity_key_hash: [u8; 20] = key.try_into().map_err(|_| {
                Error::Drive(DriveError::CorruptedCodeExecution("key hash not 20 bytes"))
            })?;
            match element {
                Some(Item(identity_id_vec, ..)) => {
                    let identity_id: [u8; 32] = identity_id_vec.try_into().map_err(|_| {
                        Error::Drive(DriveError::CorruptedCodeExecution("key hash not 20 bytes"))
                    })?;
                    Ok((identity_key_hash, Some(identity_id)))
                }
                None => Ok((identity_key_hash, None)),
                _ => Err(Error::Drive(DriveError::CorruptedDriveState(
                    "unique public key hashes containing non identity ids".to_string(),
                ))),
            }
        })
        .collect()
    }
}
