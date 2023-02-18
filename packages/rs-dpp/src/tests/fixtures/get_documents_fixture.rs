use std::sync::Arc;

use serde_json::{json, Value};

use crate::{
    contracts::withdrawals_contract,
    document::{
        document_factory::DocumentFactory,
        fetch_and_validate_data_contract::DataContractFetcherAndValidator,
    },
    prelude::*,
    state_repository::{MockStateRepositoryLike, StateRepositoryLike},
    tests::utils::generate_random_identifier_struct as gen_owner_id,
    version::LATEST_VERSION,
};

use super::get_document_validator_fixture;

pub fn get_documents_fixture_with_owner_id_from_contract(
    data_contract: DataContract,
) -> Result<Vec<Document>, ProtocolError> {
    let data_contract_fetcher_and_validator =
        DataContractFetcherAndValidator::new(Arc::new(MockStateRepositoryLike::new()));
    let factory = DocumentFactory::new(
        LATEST_VERSION,
        get_document_validator_fixture(),
        data_contract_fetcher_and_validator,
    );
    let owner_id = *data_contract.owner_id();

    get_documents(factory, data_contract, owner_id)
}

pub fn get_documents_fixture(data_contract: DataContract) -> Result<Vec<Document>, ProtocolError> {
    let data_contract_fetcher_and_validator =
        DataContractFetcherAndValidator::new(Arc::new(MockStateRepositoryLike::new()));
    let factory = DocumentFactory::new(
        LATEST_VERSION,
        get_document_validator_fixture(),
        data_contract_fetcher_and_validator,
    );
    let owner_id = gen_owner_id();

    get_documents(factory, data_contract, owner_id)
}

fn get_documents<ST: StateRepositoryLike>(
    factory: DocumentFactory<ST>,
    data_contract: DataContract,
    owner_id: Identifier,
) -> Result<Vec<Document>, ProtocolError> {
    let documents = vec![
        factory.create(
            data_contract.clone(),
            owner_id,
            "niceDocument".to_string(),
            json!({ "name": "Cutie" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "prettyDocument".to_string(),
            json!({ "lastName": "Shiny" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "prettyDocument".to_string(),
            json!({ "lastName": "Sweety" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "indexedDocument".to_string(),
            json!( { "firstName": "William", "lastName": "Birkin" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "indexedDocument".to_string(),
            json!( { "firstName": "Leon", "lastName": "Kennedy" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "noTimeDocument".to_string(),
            json!({ "name": "ImOutOfTime" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "uniqueDates".to_string(),
            json!({ "firstName": "John" }),
        )?,
        factory.create(
            data_contract.clone(),
            owner_id,
            "indexedDocument".to_string(),
            json!( { "firstName": "Bill", "lastName": "Gates" }),
        )?,
        factory.create(data_contract.clone(), owner_id, "withByteArrays".to_string(), json!( { "byteArrayField": get_random_10_bytes(), "identifierField": gen_owner_id().to_buffer() }),)?,
        factory.create(
            data_contract,
            owner_id,
            "optionalUniqueIndexedDocument".to_string(),
            json!({ "firstName": "Jacques-Yves", "lastName": "Cousteau" }),
        )?,
    ];

    Ok(documents)
}

pub fn get_withdrawal_document_fixture(
    data_contract: &DataContract,
    owner_id: Identifier,
    data: Value,
) -> Document {
    let factory = DocumentFactory::new(
        LATEST_VERSION,
        get_document_validator_fixture(),
        DataContractFetcherAndValidator::new(Arc::new(MockStateRepositoryLike::new())),
    );

    factory
        .create(
            data_contract.clone(),
            owner_id,
            withdrawals_contract::document_types::WITHDRAWAL.to_string(),
            data,
        )
        .unwrap()
}

fn get_random_10_bytes() -> Vec<u8> {
    let mut buffer = [0u8; 10];
    let _ = getrandom::getrandom(&mut buffer);
    buffer.to_vec()
}
