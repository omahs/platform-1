use std::sync::Arc;

use dapi_grpc::platform::v0::get_proofs_request::DocumentRequest;
pub use drive::drive::verify::RootHash;
pub use drive::drive::verify::{
    ContractVerifier, DataContract, Document, DocumentType, DocumentVerifier, Identity,
    IdentityVerifier, PartialIdentity, QueryVerifier,
};
use drive::{drive::Drive, query::DriveQuery, query::SingleDocumentDriveQuery};

#[cfg(feature = "mockall")]
pub use drive::drive::verify::{
    MockContractVerifier, MockDocumentVerifier, MockIdentityVerifier, MockQueryVerifier,
};

use crate::error::Error;

struct IdentityProof<'a> {
    proof: &'a [u8],
}

impl<'a> IdentityProof<'a> {
    pub fn new(proof: &'a [u8]) -> Self {
        Self { proof }
    }
    /// Verifies the full identity of a user by their public key hash.
    ///
    /// This function takes a byte slice `proof` and a 20-byte array `public_key_hash` as arguments,
    /// then it verifies the identity of the user with the given public key hash.
    ///
    /// The `proof` should contain the proof of authentication from the user.
    /// The `public_key_hash` should contain the hash of the public key of the user.
    ///
    /// The function first verifies the identity ID associated with the given public key hash
    /// by calling `verify_identity_id_by_public_key_hash()`. It then uses this identity ID to verify
    /// the full identity by calling `verify_full_identity_by_identity_id()`.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// an `Option` of `Identity`. The `RootHash` represents the root hash of GroveDB, and the
    /// `Option<Identity>` represents the full identity of the user if it exists.
    ///
    /// If the verification fails at any point, it will return an `Error`.
    ///
    /// # Errors
    ///
    /// This function will return an `Error` if:
    ///
    /// * The proof of authentication is not valid.
    /// * The public key hash does not correspond to a valid identity ID.
    /// * The identity ID does not correspond to a valid full identity.
    ///
    pub fn by_pubkey(
        proof: &[u8],
        public_key_hash: [u8; 20],
    ) -> Result<(RootHash, Option<Identity>), Error> {
        todo!()
    }

    /// Verifies the full identities of multiple users by their public key hashes.
    ///
    /// This function is a generalization of `verify_full_identity_by_public_key_hash`,
    /// which works with a slice of public key hashes instead of a single hash.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proof of authentication from the users.
    /// - `public_key_hashes`: A reference to a slice of 20-byte arrays, each representing
    ///    a hash of a public key of a user.
    ///
    /// # Generic Parameters
    ///
    /// - `T`: The type of the collection to hold the results, which must be constructible
    ///    from an iterator of tuples of a 20-byte array and an optional `Identity`.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and `T`.
    /// The `RootHash` represents the root hash of GroveDB, and `T` represents
    /// the collection of the public key hash and associated identity (if it exists) for each user.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - Any of the public key hashes do not correspond to a valid identity ID.
    /// - Any of the identity IDs do not correspond to a valid full identity.
    ///
    pub fn identity_by_pubkeys<T: FromIterator<([u8; 20], Option<Identity>)> + 'static>(
        proof: &[u8],
        public_key_hashes: &[[u8; 20]],
    ) -> Result<(RootHash, T), Error> {
        todo!()
    }

    /// Verifies the full identity of a user by their identity ID.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proof of authentication from the user.
    /// - `is_proof_subset`: A boolean indicating whether the proof is a subset.
    /// - `identity_id`: A 32-byte array representing the identity ID of the user.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// an `Option` of `Identity`. The `RootHash` represents the root hash of GroveDB, and the
    /// `Option<Identity>` represents the full identity of the user if it exists.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - The identity ID does not correspond to a valid full identity.
    /// - The balance, revision, or keys information is missing or incorrect.
    ///
    pub fn identity_by_id(
        &self,
        is_proof_subset: bool,
        identity_id: [u8; 32],
    ) -> Result<(RootHash, Option<Identity>), Error> {
        todo!()
    }

    /// Verifies the identity keys of a user by their identity ID.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proof of authentication from the user.
    /// - `is_proof_subset`: A boolean indicating whether the proof is a subset.
    /// - `identity_id`: A 32-byte array representing the identity ID of the user.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// an `Option` of `PartialIdentity`. The `RootHash` represents the root hash of GroveDB,
    /// and the `Option<PartialIdentity>` represents the partial identity of the user if it exists.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - The identity ID does not correspond to a valid partial identity.
    /// - The keys information is missing or incorrect.
    ///
    fn keys_by_id(
        proof: &[u8],
        is_proof_subset: bool,
        identity_id: [u8; 32],
    ) -> Result<(RootHash, Option<PartialIdentity>), Error>;

    /// Verifies the identity ID of a user by their public key hash.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proof of authentication from the user.
    /// - `is_proof_subset`: A boolean indicating whether the proof is a subset.
    /// - `public_key_hash`: A 20-byte array representing the hash of the public key of the user.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// an `Option` of a 32-byte array. The `RootHash` represents the root hash of GroveDB,
    /// and the `Option<[u8; 32]>` represents the identity ID of the user if it exists.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - The public key hash does not correspond to a valid identity ID.
    /// - The proved key value is not for the correct path or key in unique key hashes.
    /// - More than one identity ID is found.
    ///
    fn verify_and_retrieve_identity_id_by_pubkey(
        proof: &[u8],
        is_proof_subset: bool,
        public_key_hash: [u8; 20],
    ) -> Result<(RootHash, Option<[u8; 32]>), Error>;
    /// Verifies the balance of an identity by their identity ID.
    ///
    /// `verify_subset_of_proof` is used to indicate if we want to verify a subset of a bigger proof.
    /// For example, if the proof can prove the balance and the revision, but here we are only interested
    /// in verifying the balance.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proof of authentication from the user.
    /// - `identity_id`: A 32-byte array representing the identity ID of the user.
    /// - `verify_subset_of_proof`: A boolean indicating whether we are verifying a subset of a larger proof.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// an `Option<u64>`. The `RootHash` represents the root hash of GroveDB, and the
    /// `Option<u64>` represents the balance of the user's identity if it exists.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - The identity ID does not correspond to a valid balance.
    /// - The proved key value is not for the correct path or key in balances.
    /// - More than one balance is found.
    ///
    fn verify_and_retrieve_identity_balance_by_id(
        proof: &[u8],
        identity_id: [u8; 32],
        verify_subset_of_proof: bool,
    ) -> Result<(RootHash, Option<u64>), Error>;

    /// Verifies the balances of multiple identities by their identity IDs.
    ///
    /// `is_proof_subset` is used to indicate if we want to verify a subset of a bigger proof.
    /// For example, if the proof can prove the balances and revisions, but here we are only
    /// interested in verifying the balances.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proofs of authentication from the users.
    /// - `is_proof_subset`: A boolean indicating whether we are verifying a subset of a larger proof.
    /// - `identity_ids`: A slice of 32-byte arrays representing the identity IDs of the users.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// a generic collection `T` of tuples. Each tuple in `T` consists of a 32-byte array
    /// representing an identity ID and an `Option<Credits>`. The `Option<Credits>` represents
    /// the balance of the respective identity if it exists.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - Any of the identity IDs does not correspond to a valid balance.
    /// - The number of proved key values does not match the number of identity IDs provided.
    /// - The value size of the balance is incorrect.
    ///
    fn verify_and_retrieve_identity_balances_by_ids<
        T: FromIterator<([u8; 32], Option<Credits>)> + 'static,
    >(
        proof: &[u8],
        is_proof_subset: bool,
        identity_ids: &[[u8; 32]],
    ) -> Result<(RootHash, T), Error>;

    /// Verifies the identity IDs of multiple identities by their public key hashes.
    ///
    /// `is_proof_subset` is used to indicate if we want to verify a subset of a bigger proof.
    /// For example, if the proof can prove the identity IDs and revisions, but here we are only
    /// interested in verifying the identity IDs.
    ///
    /// # Parameters
    ///
    /// - `proof`: A byte slice representing the proofs of authentication from the users.
    /// - `is_proof_subset`: A boolean indicating whether we are verifying a subset of a larger proof.
    /// - `public_key_hashes`: A slice of 20-byte arrays representing the public key hashes of the users.
    ///
    /// # Returns
    ///
    /// If the verification is successful, it returns a `Result` with a tuple of `RootHash` and
    /// a generic collection `T` of tuples. Each tuple in `T` consists of a 20-byte array
    /// representing a public key hash and an `Option<[u8; 32]>`. The `Option<[u8; 32]>` represents
    /// the identity ID of the respective identity if it exists.
    ///
    /// # Errors
    ///
    /// Returns an `Error` if:
    ///
    /// - The proof of authentication is not valid.
    /// - Any of the public key hashes does not correspond to a valid identity ID.
    /// - The number of proved key values does not match the number of public key hashes provided.
    /// - The value size of the identity ID is incorrect.
    ///
    fn verify_and_retrieve_identity_ids_by_pubkeys<
        T: FromIterator<([u8; 20], Option<[u8; 32]>)> + 'static,
    >(
        proof: &[u8],
        is_proof_subset: bool,
        public_key_hashes: &[[u8; 20]],
    ) -> Result<(RootHash, T), Error>;
}
/// Allows verification of proofs
///
/// To verify contracts and identities, refer to [ContractVerifier] and [IdentityVerifier] traits.
///
/// To verify proof generated by a document query extract documents, use [`Verifier::for_query()`] to
/// access [DocumentVerifier] trait for that query.
pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    /// Verify proof returned by some query
    ///
    /// Note that you need to use exactly the same query as was sent to the DAPI.
    pub fn for_query<'a>(
        query: &'a DriveQuery,
    ) -> Result<Box<dyn QueryVerifier + 'a>, crate::error::Error> {
        Ok(Box::new(query.clone()))
    }

    pub fn for_document<'a>(
        doc: &DocumentRequest,
    ) -> Result<Box<dyn DocumentVerifier>, crate::error::Error> {
        let doc =
            SingleDocumentDriveQuery::try_from(doc).map_err(|e| Error::DriveError(e.into()))?;

        Ok(Box::new(doc))
    }
}

impl ContractVerifier for Verifier {
    fn verify_and_retrieve_contract(
        proof: &[u8],
        contract_known_keeps_history: Option<bool>,
        is_proof_subset: bool,
        contract_id: [u8; 32],
    ) -> Result<(drive::drive::verify::RootHash, Option<DataContract>), drive::error::Error> {
        Drive::verify_and_retrieve_contract(
            proof,
            contract_known_keeps_history,
            is_proof_subset,
            contract_id,
        )
    }
    fn verify_and_retrieve_contract_history(
        proof: &[u8],
        contract_id: [u8; 32],
        start_at_date: u64,
        limit: Option<u16>,
        offset: Option<u16>,
    ) -> Result<
        (
            drive::drive::verify::RootHash,
            Option<std::collections::BTreeMap<u64, DataContract>>,
        ),
        drive::error::Error,
    > {
        Drive::verify_and_retrieve_contract_history(
            proof,
            contract_id,
            start_at_date,
            limit,
            offset,
        )
    }
}

// impl IdentityVerifier for Verifier {}
