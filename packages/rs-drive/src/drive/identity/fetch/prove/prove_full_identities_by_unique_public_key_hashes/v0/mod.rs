use crate::drive::Drive;

use crate::error::Error;

use dpp::version::drive_versions::DriveVersion;
use grovedb::{PathQuery, TransactionArg};
use dpp::version::PlatformVersion;

impl Drive {
    /// Given public key hashes, fetches full identities as proofs.
    pub(super) fn prove_full_identities_by_unique_public_key_hashes_v0(
        &self,
        public_key_hashes: &[[u8; 20]],
        transaction: TransactionArg,
        platform_version: &PlatformVersion,
    ) -> Result<Vec<u8>, Error> {
        let identity_ids = self.fetch_identity_ids_by_unique_public_key_hashes(
            public_key_hashes,
            transaction,
            platform_version,
        )?;
        let path_queries = identity_ids
            .into_iter()
            .map(|(public_key_hash, maybe_identity_id)| {
                if let Some(identity_id) = maybe_identity_id {
                    Self::full_identity_with_public_key_hash_query(public_key_hash, identity_id)
                } else {
                    Ok(Self::identity_id_by_unique_public_key_hash_query(
                        public_key_hash,
                    ))
                }
            })
            .collect::<Result<Vec<PathQuery>, Error>>()?;

        let path_query = PathQuery::merge(path_queries.iter().collect()).map_err(Error::GroveDB)?;
        self.grove_get_proved_path_query(&path_query, true, transaction, &mut vec![], &platform_version.drive)
    }
}

#[cfg(test)]
mod tests {
    use crate::drive::Drive;
    use crate::tests::helpers::setup::setup_drive_with_initial_state_structure;
    use dpp::block::block_info::BlockInfo;
    use dpp::identity::Identity;
    use std::collections::BTreeMap;
    use dpp::identity::accessors::IdentityGettersV0;
    use dpp::version::PlatformVersion;

    #[test]
    fn should_prove_multiple_identities() {
        let drive = setup_drive_with_initial_state_structure();
        
        let platform_version = PlatformVersion::latest();

        let identities: BTreeMap<[u8; 32], Identity> =
            Identity::random_identities( 10, 3, Some(14), platform_version)
                .into_iter()
                .map(|identity| (identity.id().to_buffer(), identity))
                .collect();

        for identity in identities.values() {
            drive
                .add_new_identity(identity.clone(), &BlockInfo::default(), true, None, platform_version)
                .expect("expected to add an identity");
        }

        let key_hashes_to_identities = identities
            .values()
            .flat_map(|identity| {
                identity
                    .public_keys()
                    .values()
                    .filter(|public_key| public_key.key_type.is_unique_key_type())
                    .map(move |public_key| {
                        (
                            public_key
                                .hash()
                                .expect("expected to hash data")
                                .try_into()
                                .expect("expected to be 20 bytes"),
                            Some(identity.clone()),
                        )
                    })
            })
            .collect::<BTreeMap<[u8; 20], Option<Identity>>>();

        let key_hashes_to_identity_ids = identities
            .values()
            .flat_map(|identity| {
                identity
                    .public_keys()
                    .values()
                    .filter(|public_key| public_key.key_type.is_unique_key_type())
                    .map(move |public_key| {
                        (
                            public_key
                                .hash()
                                .expect("expected to hash data")
                                .try_into()
                                .expect("expected to be 20 bytes"),
                            Some(identity.id().to_buffer()),
                        )
                    })
            })
            .collect::<BTreeMap<[u8; 20], Option<[u8; 32]>>>();

        let key_hashes = key_hashes_to_identity_ids
            .keys()
            .copied()
            .collect::<Vec<[u8; 20]>>();

        let proof = drive
            .prove_full_identities_by_unique_public_key_hashes(&key_hashes, None, platform_version)
            .expect("should not error when proving an identity");

        let (_, proved_identity_ids): ([u8; 32], BTreeMap<[u8; 20], Option<Identity>>) =
            Drive::verify_full_identities_by_public_key_hashes(proof.as_slice(), &key_hashes)
                .expect("expect that this be verified");

        assert_eq!(proved_identity_ids, key_hashes_to_identities);
    }
}
