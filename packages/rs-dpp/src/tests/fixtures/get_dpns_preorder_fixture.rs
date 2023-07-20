use std::collections::BTreeMap;
use std::sync::Arc;

use getrandom::getrandom;
use platform_value::Value;

use crate::document::ExtendedDocument;
use crate::tests::fixtures::ParentDocumentOptions;
use crate::util::hash::hash;
use crate::{
    document::{
        document_factory::DocumentFactory,
        fetch_and_validate_data_contract::DataContractFetcherAndValidator,
    },
    state_repository::MockStateRepositoryLike,
    version::LATEST_VERSION,
};

use super::{get_document_validator_fixture, get_dpns_data_contract_fixture};

pub fn get_dpns_preorder_document_fixture(
    options: ParentDocumentOptions,
) -> (ExtendedDocument, [u8; 32]) {
    let document_factory = DocumentFactory::new(
        LATEST_VERSION,
        get_document_validator_fixture(),
        DataContractFetcherAndValidator::new(Arc::new(MockStateRepositoryLike::new())),
    );
    let data_contract = get_dpns_data_contract_fixture(Some(options.owner_id));
    let mut pre_order_salt = [0u8; 32];
    let _ = getrandom(&mut pre_order_salt);

    let salted_domain_hash = hash(&[&options.label, &pre_order_salt].concat());

    let mut map = BTreeMap::new();
    map.insert(
        "saltedDomainHash".to_string(),
        Value::Bytes32(salted_domain_hash),
    );

    (
        document_factory
            .create_extended_document_for_state_transition(
                data_contract.data_contract,
                options.owner_id,
                String::from("preorder"),
                map.into(),
            )
            .expect("DPNS preorder document should be created"),
        pre_order_salt,
    )
}
