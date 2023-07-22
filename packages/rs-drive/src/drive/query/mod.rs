// MIT LICENSE
//
// Copyright (c) 2021 Dash Core Group
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

//! Drive Queries
//!
//! Defines and implements in Drive functions relevant to querying.
//!

mod query_document_ids;
mod query_documents;
mod query_documents_cbor_from_contract;
mod query_documents_cbor_with_document_type_lookup;
mod query_documents_for_cbor_query_internal;
mod query_documents_from_contract;
mod query_proof_of_documents_using_cbor_encoded_query;
mod query_proof_of_documents_using_cbor_encoded_query_only_get_elements;
mod query_proof_of_documents_using_contract_id_using_cbor_encoded_query_with_cost;
mod query_raw_documents_from_contract_cbor_using_cbor_encoded_query_with_cost;

use grovedb::query_result_type::{Key, QueryResultType};
use grovedb::TransactionArg;

use crate::drive::Drive;
use crate::error::query::QuerySyntaxError;
use crate::error::Error;
use crate::fee::op::LowLevelDriveOperation;
use crate::query::DriveQuery;
use dpp::data_contract::document_type::DocumentTypeRef;
use dpp::data_contract::DataContract;

use dpp::document::Document;

use dpp::ProtocolError;

use crate::drive::system::protocol_version;
use dpp::block::block_info::BlockInfo;
use dpp::block::epoch::Epoch;
use dpp::data_contract::base::DataContractBaseMethodsV0;
#[cfg(test)]
use dpp::data_contract::conversion::cbor_conversion::DataContractCborConversionMethodsV0;
use dpp::document::serialization_traits::DocumentPlatformConversionMethodsV0;
use dpp::version::PlatformVersion;

#[derive(Debug, Default)]
/// The outcome of a query
pub struct QueryDocumentsOutcome {
    /// returned items
    pub documents: Vec<Document>,
    /// skipped item count
    pub skipped: u16,
    /// the processing cost
    pub cost: u64,
}

/// The outcome of a query
pub struct QuerySerializedDocumentsOutcome {
    /// returned items
    pub items: Vec<Vec<u8>>,
    /// skipped item count
    pub skipped: u16,
    /// the processing cost
    pub cost: u64,
}

/// The outcome of a query
pub struct QueryDocumentIdsOutcome {
    /// returned items
    pub items: Vec<Key>,
    /// skipped item count
    pub skipped: u16,
    /// the processing cost
    pub cost: u64,
}

impl Drive {
    /// Performs and returns the result of the specified query along with skipped items
    /// and the cost.
    pub fn query_documents(
        &self,
        query: DriveQuery,
        epoch: Option<&Epoch>,
        dry_run: bool,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<QueryDocumentsOutcome, Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        if dry_run {
            return Ok(QueryDocumentsOutcome::default());
        }
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let (items, skipped) = query.execute_raw_results_no_proof_internal(
            self,
            transaction,
            &mut drive_operations,
            platform_version,
        )?;
        let documents = items
            .into_iter()
            .map(|serialized| {
                Document::from_bytes(serialized.as_slice(), query.document_type, platform_version)
            })
            .collect::<Result<Vec<Document>, ProtocolError>>()?;
        let cost = if let Some(epoch) = epoch {
            let fee_result =
                Drive::calculate_fee(None, Some(drive_operations), epoch, platform_version)?;
            fee_result.processing_fee
        } else {
            0
        };

        Ok(QueryDocumentsOutcome {
            documents,
            skipped,
            cost,
        })
    }

    /// Performs and returns the result of the specified query along with skipped items
    /// and the cost.
    pub fn query_documents_as_serialized(
        &self,
        query: DriveQuery,
        epoch: Option<&Epoch>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<QuerySerializedDocumentsOutcome, Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let (items, skipped) = query.execute_raw_results_no_proof_internal(
            self,
            transaction,
            &mut drive_operations,
            platform_version,
        )?;
        let cost = if let Some(epoch) = epoch {
            let fee_result =
                Drive::calculate_fee(None, Some(drive_operations), epoch, platform_version)?;
            fee_result.processing_fee
        } else {
            0
        };

        Ok(QuerySerializedDocumentsOutcome {
            items,
            skipped,
            cost,
        })
    }

    /// Performs and returns the result as ids of the specified query
    /// along with skipped items and the cost.
    pub fn query_document_ids(
        &self,
        query: DriveQuery,
        epoch: Option<&Epoch>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<QueryDocumentIdsOutcome, Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let (items, skipped) = query.execute_no_proof_internal(
            self,
            QueryResultType::QueryKeyElementPairResultType,
            transaction,
            &mut drive_operations,
            platform_version,
        )?;
        let cost = if let Some(epoch) = epoch {
            let fee_result =
                Drive::calculate_fee(None, Some(drive_operations), epoch, platform_version)?;
            fee_result.processing_fee
        } else {
            0
        };

        let keys = items
            .to_key_elements()
            .into_iter()
            .map(|(key, _element)| key)
            .collect();

        Ok(QueryDocumentIdsOutcome {
            items: keys,
            skipped,
            cost,
        })
    }
    /// Performs and returns the result of the specified query along with skipped items and the cost.
    pub fn query_documents_cbor_with_document_type_lookup(
        &self,
        query_cbor: &[u8],
        contract_id: [u8; 32],
        document_type_name: &str,
        epoch: Option<&Epoch>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<QuerySerializedDocumentsOutcome, Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let contract = self
            .get_contract_with_fetch_info_and_add_to_operations(
                contract_id,
                epoch,
                true,
                transaction,
                &mut drive_operations,
                platform_version,
            )?
            .ok_or(Error::Query(QuerySyntaxError::DataContractNotFound(
                "contract not found",
            )))?;
        let document_type = contract
            .contract
            .document_type_for_name(document_type_name)?;

        let query =
            DriveQuery::from_cbor(query_cbor, &contract.contract, document_type, &self.config)?;

        self.query_documents_as_serialized(query, epoch, transaction, protocol_version)
    }

    #[cfg(test)]
    /// Performs and returns the result of the specified query along with skipped items and the cost.
    pub fn query_raw_documents_from_contract_cbor_using_cbor_encoded_query_with_cost(
        &self,
        query_cbor: &[u8],
        contract_cbor: &[u8],
        document_type_name: String,
        block_info: Option<BlockInfo>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<(Vec<Vec<u8>>, u16, u64), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let contract = DataContract::from_cbor(contract_cbor, platform_version)?;
        //todo cbor cost
        let document_type = contract.document_type_for_name(document_type_name.as_str())?;

        let (items, skipped) = self.query_documents_for_cbor_query_internal(
            &contract,
            document_type,
            query_cbor,
            transaction,
            &mut drive_operations,
            protocol_version,
        )?;
        let cost = if let Some(block_info) = block_info {
            let fee_result = Drive::calculate_fee(
                None,
                Some(drive_operations),
                &block_info.epoch,
                platform_version,
            )?;
            fee_result.processing_fee
        } else {
            0
        };
        Ok((items, skipped, cost))
    }

    /// Performs and returns the result of the specified query along with skipped items and the cost.
    pub fn query_documents_cbor_from_contract(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        block_info: Option<BlockInfo>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<(Vec<Vec<u8>>, u16, u64), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let (items, skipped) = self.query_documents_for_cbor_query_internal(
            contract,
            document_type,
            query_cbor,
            transaction,
            &mut drive_operations,
            protocol_version,
        )?;
        let cost = if let Some(block_info) = block_info {
            let fee_result = Drive::calculate_fee(
                None,
                Some(drive_operations),
                &block_info.epoch,
                platform_version,
            )?;
            fee_result.processing_fee
        } else {
            0
        };
        Ok((items, skipped, cost))
    }

    /// Performs and returns the result of the specified query along with skipped items and the cost.
    pub fn query_documents_from_contract(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        block_info: Option<BlockInfo>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<(Vec<Vec<u8>>, u16, u64), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let (items, skipped) = self.query_documents_for_cbor_query_internal(
            contract,
            document_type,
            query_cbor,
            transaction,
            &mut drive_operations,
            protocol_version,
        )?;
        let cost = if let Some(block_info) = block_info {
            let fee_result = Drive::calculate_fee(
                None,
                Some(drive_operations),
                &block_info.epoch,
                platform_version,
            )?;
            fee_result.processing_fee
        } else {
            0
        };
        Ok((items, skipped, cost))
    }

    /// Performs and returns the result of the specified query along with skipped items.
    pub(crate) fn query_documents_for_cbor_query_internal(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        protocol_version: Option<u32>,
    ) -> Result<(Vec<Vec<u8>>, u16), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let query = DriveQuery::from_cbor(query_cbor, contract, document_type, &self.config)?;

        query.execute_raw_results_no_proof_internal(
            self,
            transaction,
            drive_operations,
            platform_version,
        )
    }

    /// Performs and returns the result of the specified query along with the fee.
    /// Proof is generated.
    pub fn query_proof_of_documents_using_contract_id_using_cbor_encoded_query_with_cost(
        &self,
        query_cbor: &[u8],
        contract_id: [u8; 32],
        document_type_name: &str,
        block_info: Option<BlockInfo>,
        epoch: Option<&Epoch>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<(Vec<u8>, u64), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];
        let contract = self
            .get_contract_with_fetch_info_and_add_to_operations(
                contract_id,
                epoch,
                true,
                transaction,
                &mut drive_operations,
                platform_version,
            )?
            .ok_or(Error::Query(QuerySyntaxError::DataContractNotFound(
                "contract not found",
            )))?;
        let document_type = contract
            .contract
            .document_type_for_name(document_type_name)?;
        let items = self.query_proof_of_documents_using_cbor_encoded_query(
            &contract.contract,
            document_type,
            query_cbor,
            transaction,
            &mut drive_operations,
            protocol_version,
        )?;
        let cost = if let Some(block_info) = block_info {
            let fee_result = Drive::calculate_fee(
                None,
                Some(drive_operations),
                &block_info.epoch,
                platform_version,
            )?;
            fee_result.processing_fee
        } else {
            0
        };
        Ok((items, cost))
    }

    /// Performs and returns the result of the specified query along with the fee.
    /// Proof is generated.
    pub fn query_proof_of_documents_using_cbor_encoded_query_with_cost(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        block_info: Option<BlockInfo>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<(Vec<u8>, u64), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];

        let items = self.query_proof_of_documents_using_cbor_encoded_query(
            contract,
            document_type,
            query_cbor,
            transaction,
            &mut drive_operations,
            protocol_version,
        )?;
        let cost = if let Some(block_info) = block_info {
            let fee_result = Drive::calculate_fee(
                None,
                Some(drive_operations),
                &block_info.epoch,
                platform_version,
            )?;
            fee_result.processing_fee
        } else {
            0
        };
        Ok((items, cost))
    }

    /// Performs and returns the result of the specified internal query.
    /// Proof is generated.
    pub(crate) fn query_proof_of_documents_using_cbor_encoded_query(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        protocol_version: Option<u32>,
    ) -> Result<Vec<u8>, Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let query = DriveQuery::from_cbor(query_cbor, contract, document_type, &self.config)?;

        query.execute_with_proof_internal(self, transaction, drive_operations, platform_version)
    }

    /// Performs the specified internal query and returns the root hash, values, and fee.
    pub fn query_proof_of_documents_using_cbor_encoded_query_only_get_elements(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        block_info: Option<BlockInfo>,
        transaction: TransactionArg,
        protocol_version: Option<u32>,
    ) -> Result<([u8; 32], Vec<Vec<u8>>, u64), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];

        let (root_hash, items) = self
            .query_proof_of_documents_using_cbor_encoded_query_only_get_elements_internal(
                contract,
                document_type,
                query_cbor,
                transaction,
                &mut drive_operations,
                protocol_version,
            )?;
        let cost = if let Some(block_info) = block_info {
            let fee_result = Drive::calculate_fee(
                None,
                Some(drive_operations),
                &block_info.epoch,
                platform_version,
            )?;
            fee_result.processing_fee
        } else {
            0
        };
        Ok((root_hash, items, cost))
    }

    /// Performs the specified internal query and returns the root hash and values.
    pub(crate) fn query_proof_of_documents_using_cbor_encoded_query_only_get_elements_internal(
        &self,
        contract: &DataContract,
        document_type: DocumentTypeRef,
        query_cbor: &[u8],
        transaction: TransactionArg,
        drive_operations: &mut Vec<LowLevelDriveOperation>,
        protocol_version: Option<u32>,
    ) -> Result<([u8; 32], Vec<Vec<u8>>), Error> {
        let platform_version = PlatformVersion::get_version_or_current_or_latest(protocol_version)?;
        let query = DriveQuery::from_cbor(query_cbor, contract, document_type, &self.config)?;

        query.execute_with_proof_only_get_elements_internal(
            self,
            transaction,
            drive_operations,
            platform_version,
        )
    }
}
