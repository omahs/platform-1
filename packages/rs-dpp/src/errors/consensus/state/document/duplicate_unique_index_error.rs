use crate::consensus::state::state_error::StateError;
use crate::consensus::ConsensusError;
use bincode::{Decode, Encode};
use platform_value::Identifier;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[error("Document {document_id} has duplicate unique properties {duplicating_properties:?} with other documents")]
pub struct DuplicateUniqueIndexError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    document_id: Identifier,
    duplicating_properties: Vec<String>,
}

impl DuplicateUniqueIndexError {
    pub fn new(document_id: Identifier, duplicating_properties: Vec<String>) -> Self {
        Self {
            document_id,
            duplicating_properties,
        }
    }

    pub fn document_id(&self) -> &Identifier {
        &self.document_id
    }

    pub fn duplicating_properties(&self) -> &Vec<String> {
        &self.duplicating_properties
    }
}

impl From<DuplicateUniqueIndexError> for ConsensusError {
    fn from(err: DuplicateUniqueIndexError) -> Self {
        Self::StateError(StateError::DuplicateUniqueIndexError(err))
    }
}
