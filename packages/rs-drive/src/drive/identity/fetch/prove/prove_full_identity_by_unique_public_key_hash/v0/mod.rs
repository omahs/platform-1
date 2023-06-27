use crate::drive::Drive;

use crate::error::Error;

use grovedb::{PathQuery, TransactionArg};
use dpp::version::drive_versions::DriveVersion;

impl Drive {
    /// Fetches an identity with all its information from storage.
    pub(super) fn prove_full_identity_by_unique_public_key_hash_v0(
        &self,
        public_key_hash: [u8; 20],
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<Vec<u8>, Error> {
        let identity_id = self.fetch_identity_id_by_unique_public_key_hash_operations(
            public_key_hash,
            transaction,
            &mut vec![],
            drive_version,
        )?;
        if let Some(identity_id) = identity_id {
            let query =
                Self::full_identity_with_public_key_hash_query(public_key_hash, identity_id)?;
            self.grove_get_proved_path_query(&query, true, transaction, &mut vec![], drive_version)
        } else {
            // We only prove the absence of the public key hash
            let query = Self::identity_id_by_unique_public_key_hash_query(public_key_hash);
            self.grove_get_proved_path_query(&query, false, transaction, &mut vec![], drive_version)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::helpers::setup::setup_drive_with_initial_state_structure;
    use dpp::block::extended_block_info::BlockInfo;
    use dpp::identity::Identity;
    use std::collections::BTreeMap;


        #[test]
        fn should_prove_a_single_identity() {
            let drive = setup_drive_with_initial_state_structure();
            let identity = Identity::random_identity(Some(0), 3, Some(14));

            drive
                .add_new_identity(identity.clone(), &BlockInfo::default(), true, None)
                .expect("expected to add an identity");

            let first_key_hash = identity
                .public_keys()
                .values()
                .find(|public_key| public_key.key_type.is_unique_key_type())
                .expect("expected a unique key")
                .hash()
                .expect("expected to hash data")
                .try_into()
                .expect("expected to be 20 bytes");

            let proof = drive
                .prove_full_identity_by_unique_public_key_hash(first_key_hash, None)
                .expect("should not error when proving an identity");

            let (_, proved_identity) =
                Drive::verify_full_identity_by_public_key_hash(proof.as_slice(), first_key_hash)
                    .expect("expect that this be verified");

            assert_eq!(proved_identity, Some(identity));
        }
}