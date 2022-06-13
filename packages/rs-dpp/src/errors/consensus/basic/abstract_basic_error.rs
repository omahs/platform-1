use crate::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BasicError {
    #[error("Data Contract {data_contract_id} is not present")]
    DataContractContPresent { data_contract_id: Identifier },

    #[error("$type is not present")]
    MissingDocumentTypeError,

    #[error("Data Contract version must be {expected_version}, go {version}")]
    InvalidDataContractVersionError { expected_version: u32, version: u32 },

    #[error("JSON Schema depth is greater than {0}")]
    DataContractMaxDepthExceedError(usize),

    // Document
    #[error(
        "Data Contract {data_contract_id} doesn't define document with the type {document_type}"
    )]
    InvalidDocumentTypeError {
        document_type: String,
        data_contract_id: Identifier,
    },
}
