mod v0;

use crate::contract::Contract;
use crate::drive::Drive;
use crate::error::Error;
use crate::query::{DriveQuery, InternalClauses, WhereClause, WhereOperator};
use dpp::consensus::state::document::duplicate_unique_index_error::DuplicateUniqueIndexError;
use dpp::consensus::state::state_error::StateError;
use dpp::data_contract::document_type::DocumentType;
use dpp::document::Document;
use dpp::identifier::Identifier;
use dpp::platform_value::{platform_value, Value};
use dpp::prelude::TimestampMillis;
use dpp::validation::SimpleConsensusValidationResult;
use grovedb::TransactionArg;
use std::collections::BTreeMap;
use dpp::data_contract::DataContract;
use dpp::state_transition::documents_batch_transition::document_transition::{DocumentCreateTransitionAction, DocumentReplaceTransitionAction};
use dpp::version::drive_versions::DriveVersion;
use crate::error::drive::DriveError;

impl Drive {
    /// Validate that a document create transition action would be unique in the state.
    ///
    /// # Arguments
    ///
    /// * `contract` - A `DataContract` object representing the contract.
    /// * `document_type` - A `DocumentType` object representing the type of the document.
    /// * `document_create_transition` - A `DocumentCreateTransitionAction` object representing the document create transition action.
    /// * `owner_id` - An `Identifier` object representing the owner's ID.
    /// * `transaction` - A `TransactionArg` object representing the transaction.
    /// * `drive_version` - A `DriveVersion` object representing the version of the Drive.
    ///
    /// # Returns
    ///
    /// * `Result<SimpleConsensusValidationResult, Error>` - If successful, returns a `SimpleConsensusValidationResult` object representing the result of the validation.
    ///   If an error occurs during the operation, returns an `Error`.
    ///
    /// # Errors
    ///
    /// This function will return an error if the version of the Drive is unknown.
    pub fn validate_document_create_transition_action_uniqueness(
        &self,
        contract: &DataContract,
        document_type: &DocumentType,
        document_create_transition: &DocumentCreateTransitionAction,
        owner_id: &Identifier,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<SimpleConsensusValidationResult, Error> {
        match drive_version.methods.document.index_uniqueness.validate_document_create_transition_action_uniqueness {
            0 => self.validate_document_create_transition_action_uniqueness_v0(contract, document_type, document_create_transition, owner_id, transaction, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "validate_document_create_transition_action_uniqueness".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}