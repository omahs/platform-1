use std::collections::BTreeMap;

use dpp::data_contract::document_type::random_document::CreateRandomDocument;
use dpp::{
    contracts::withdrawals_contract, data_contract::DriveContractExt,
    document::document_stub::DocumentStub,
};
use grovedb::TransactionArg;
use indexmap::IndexMap;
use lazy_static::__Deref;

use crate::{
    drive::{query::QueryDocumentsOutcome, Drive},
    error::{drive::DriveError, Error},
    query::{DriveQuery, InternalClauses, OrderClause, WhereClause},
};

impl Drive {
    /// Fetch withdrawal documents by it's status
    pub fn fetch_withdrawal_documents_by_status(
        &self,
        status: u8,
        transaction: TransactionArg,
    ) -> Result<Vec<DocumentStub>, Error> {
        let data_contract_id = withdrawals_contract::CONTRACT_ID.deref();

        let contract_fetch_info = self
            .get_contract_with_fetch_info(data_contract_id.to_buffer(), None, transaction)?
            .1
            .ok_or_else(|| {
                Error::Drive(DriveError::CorruptedCodeExecution(
                    "Can't fetch data contract",
                ))
            })?;

        let document_type = contract_fetch_info
            .contract
            .document_type_for_name(withdrawals_contract::document_types::WITHDRAWAL)?;

        let mut where_clauses = BTreeMap::new();

        //todo: make this lazy loaded or const
        where_clauses.insert(
            withdrawals_contract::property_names::STATUS.to_string(),
            WhereClause {
                field: withdrawals_contract::property_names::STATUS.to_string(),
                operator: crate::query::WhereOperator::Equal,
                value: ciborium::Value::Integer(status.into()),
            },
        );

        let mut order_by = IndexMap::new();

        order_by.insert(
            withdrawals_contract::property_names::UPDATED_AT.to_string(),
            OrderClause {
                field: withdrawals_contract::property_names::UPDATED_AT.to_string(),
                ascending: true,
            },
        );

        let drive_query = DriveQuery {
            contract: &contract_fetch_info.contract,
            document_type,
            internal_clauses: InternalClauses {
                primary_key_in_clause: None,
                primary_key_equal_clause: None,
                in_clause: None,
                range_clause: None,
                equal_clauses: where_clauses,
            },
            offset: 0,
            limit: 100,
            order_by,
            start_at: None,
            start_at_included: false,
            block_time: None,
        };

        let QueryDocumentsOutcome {
            items,
            skipped: _,
            cost: _,
        } = self.query_documents(drive_query, None, transaction)?;

        let documents = items
            .iter()
            .map(|document_cbor| {
                document_type
                    .document_from_bytes(document_cbor)
                    .map_err(|e| {
                        Error::Drive(DriveError::CorruptedDriveState(format!(
                            "can't create document from bytes : {e}"
                        )))
                    })
            })
            .collect::<Result<Vec<DocumentStub>, Error>>()?;

        Ok(documents)
    }

    /// Find one document by it's transactionId field
    pub fn find_withdrawal_document_by_transaction_id(
        &self,
        original_transaction_id: &[u8],
        transaction: TransactionArg,
    ) -> Result<DocumentStub, Error> {
        let data_contract_id = withdrawals_contract::CONTRACT_ID.deref();

        let contract_fetch_info = self
            .get_contract_with_fetch_info(data_contract_id.to_buffer(), None, transaction)?
            .1
            .ok_or_else(|| {
                Error::Drive(DriveError::CorruptedCodeExecution(
                    "Can't fetch data contract",
                ))
            })?;

        let document_type = contract_fetch_info
            .contract
            .document_type_for_name(withdrawals_contract::document_types::WITHDRAWAL)?;

        let mut where_clauses = BTreeMap::new();

        where_clauses.insert(
            withdrawals_contract::property_names::TRANSACTION_ID.to_string(),
            WhereClause {
                field: withdrawals_contract::property_names::TRANSACTION_ID.to_string(),
                operator: crate::query::WhereOperator::Equal,
                value: ciborium::Value::Bytes(original_transaction_id.to_vec()),
            },
        );

        where_clauses.insert(
            withdrawals_contract::property_names::STATUS.to_string(),
            WhereClause {
                field: withdrawals_contract::property_names::STATUS.to_string(),
                operator: crate::query::WhereOperator::Equal,
                value: ciborium::Value::Integer(
                    (withdrawals_contract::WithdrawalStatus::POOLED as u8).into(),
                ),
            },
        );

        let drive_query = DriveQuery {
            contract: &contract_fetch_info.contract,
            document_type,
            internal_clauses: InternalClauses {
                primary_key_in_clause: None,
                primary_key_equal_clause: None,
                in_clause: None,
                range_clause: None,
                equal_clauses: where_clauses,
            },
            offset: 0,
            limit: 100,
            order_by: IndexMap::new(),
            start_at: None,
            start_at_included: false,
            block_time: None,
        };

        let QueryDocumentsOutcome {
            items,
            skipped: _,
            cost: _,
        } = self.query_documents(drive_query, None, transaction)?;

        let documents = items
            .iter()
            .map(|document_cbor| {
                DocumentStub::from_bytes(document_cbor, document_type).map_err(|_| {
                    Error::Drive(DriveError::CorruptedDriveState(
                        "can't create document from bytes".to_string(),
                    ))
                })
            })
            .collect::<Result<Vec<DocumentStub>, Error>>()?;

        let document = documents
            .get(0)
            .ok_or(Error::Drive(DriveError::CorruptedCodeExecution(
                "document was not found by transactionId",
            )))?
            .clone();

        Ok(document)
    }
}

#[cfg(test)]
mod tests {
    use dpp::contracts::withdrawals_contract;
    use dpp::prelude::Identifier;
    use dpp::tests::fixtures::get_withdrawal_document_fixture;
    use serde_json::json;

    use crate::tests::helpers::setup::setup_drive_with_initial_state_structure;
    use crate::tests::helpers::setup::{setup_document, setup_system_data_contract};

    mod fetch_withdrawal_documents_by_status {
        use dpp::data_contract::DriveContractExt;
        use dpp::identity::state_transition::identity_credit_withdrawal_transition::Pooling;
        use dpp::system_data_contracts::{load_system_data_contract, SystemDataContract};

        use super::*;

        #[test]
        fn test_return_list_of_documents() {
            let drive = setup_drive_with_initial_state_structure();

            let transaction = drive.grove.start_transaction();

            let data_contract = load_system_data_contract(SystemDataContract::Withdrawals)
                .expect("to load system data contract");

            setup_system_data_contract(&drive, &data_contract, Some(&transaction));

            let documents = drive
                .fetch_withdrawal_documents_by_status(
                    withdrawals_contract::WithdrawalStatus::QUEUED.into(),
                    Some(&transaction),
                )
                .expect("to fetch documents by status");

            assert_eq!(documents.len(), 0);

            let owner_id = Identifier::new([1u8; 32]);

            let document = get_withdrawal_document_fixture(
                &data_contract,
                owner_id,
                json!({
                    "amount": 1000,
                    "coreFeePerByte": 1,
                    "pooling": Pooling::Never,
                    "outputScript": (0..23).collect::<Vec<u8>>(),
                    "status": withdrawals_contract::WithdrawalStatus::QUEUED,
                    "transactionIndex": 1,
                }),
            );

            let document_type = data_contract
                .document_type_for_name(withdrawals_contract::document_types::WITHDRAWAL)
                .expect("expected to get document type");

            setup_document(
                &drive,
                &document,
                &data_contract,
                document_type,
                Some(&transaction),
            );

            let document = get_withdrawal_document_fixture(
                &data_contract,
                owner_id,
                json!({
                    "amount": 1000,
                    "coreFeePerByte": 1,
                    "pooling": Pooling::Never,
                    "outputScript": (0..23).collect::<Vec<u8>>(),
                    "status": withdrawals_contract::WithdrawalStatus::POOLED,
                    "transactionIndex": 2,
                }),
            );

            setup_document(
                &drive,
                &document,
                &data_contract,
                document_type,
                Some(&transaction),
            );

            let documents = drive
                .fetch_withdrawal_documents_by_status(
                    withdrawals_contract::WithdrawalStatus::QUEUED.into(),
                    Some(&transaction),
                )
                .expect("to fetch documents by status");

            assert_eq!(documents.len(), 1);

            let documents = drive
                .fetch_withdrawal_documents_by_status(
                    withdrawals_contract::WithdrawalStatus::POOLED.into(),
                    Some(&transaction),
                )
                .expect("to fetch documents by status");

            assert_eq!(documents.len(), 1);
        }
    }

    mod find_document_by_transaction_id {
        use dpp::data_contract::DriveContractExt;
        use dpp::identity::state_transition::identity_credit_withdrawal_transition::Pooling;
        use dpp::system_data_contracts::{load_system_data_contract, SystemDataContract};

        use super::*;

        #[test]
        fn test_find_document_by_transaction_id() {
            let drive = setup_drive_with_initial_state_structure();

            let transaction = drive.grove.start_transaction();

            let data_contract = load_system_data_contract(SystemDataContract::Withdrawals)
                .expect("to load system data contract");

            setup_system_data_contract(&drive, &data_contract, Some(&transaction));

            let owner_id = Identifier::new([1u8; 32]);

            let document = get_withdrawal_document_fixture(
                &data_contract,
                owner_id,
                json!({
                    "amount": 1000,
                    "coreFeePerByte": 1,
                    "pooling": Pooling::Never,
                    "outputScript": (0..23).collect::<Vec<u8>>(),
                    "status": withdrawals_contract::WithdrawalStatus::POOLED,
                    "transactionIndex": 1,
                    "transactionId": (0..32).collect::<Vec<u8>>(),
                }),
            );

            let document_type = data_contract
                .document_type_for_name(withdrawals_contract::document_types::WITHDRAWAL)
                .expect("expected to get document type");

            setup_document(
                &drive,
                &document,
                &data_contract,
                document_type,
                Some(&transaction),
            );

            let found_document = drive
                .find_withdrawal_document_by_transaction_id(
                    &(0..32).collect::<Vec<u8>>(),
                    Some(&transaction),
                )
                .expect("to find document by it's transaction id");

            assert_eq!(found_document.id.to_vec(), document.id.to_vec());
        }
    }
}
