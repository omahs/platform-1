use std::borrow::Cow;
use grovedb::TransactionArg;
use serde::Deserialize;
use dpp::block::block_info::BlockInfo;
use dpp::document::Document;
use dpp::version::drive_versions::DriveVersion;
use crate::contract::Contract;
use crate::drive::Drive;
use crate::drive::flags::StorageFlags;
use crate::drive::object_size_info::{DocumentAndContractInfo, OwnedDocumentInfo};
use crate::drive::object_size_info::DocumentInfo::DocumentRefAndSerialization;
use crate::error::document::DocumentError;
use crate::error::Error;
use crate::fee::calculate_fee;
use crate::fee::op::LowLevelDriveOperation;
use crate::fee::result::FeeResult;


impl Drive {
    /// Deserializes a document and adds it to a contract by id.
    pub(super) fn add_serialized_document_for_contract_id_v0(
        &self,
        serialized_document: &[u8],
        contract_id: [u8; 32],
        document_type_name: &str,
        owner_id: Option<[u8; 32]>,
        override_document: bool,
        block_info: BlockInfo,
        apply: bool,
        storage_flags: Option<Cow<StorageFlags>>,
        transaction: TransactionArg,
        drive_version: &DriveVersion,
    ) -> Result<FeeResult, Error> {
        let mut drive_operations: Vec<LowLevelDriveOperation> = vec![];

        let contract_fetch_info = self
            .get_contract_with_fetch_info_and_add_to_operations(
                contract_id,
                Some(&block_info.epoch),
                true,
                transaction,
                &mut drive_operations,
            )?
            .ok_or(Error::Document(DocumentError::ContractNotFound))?;

        let contract = &contract_fetch_info.contract;

        let document = Document::from_cbor(serialized_document, None, owner_id)?;

        let document_info =
            DocumentRefAndSerialization((&document, serialized_document, storage_flags));

        let document_type = contract.document_type_for_name(document_type_name)?;

        self.add_document_for_contract_apply_and_add_to_operations(
            DocumentAndContractInfo {
                owned_document_info: OwnedDocumentInfo {
                    document_info,
                    owner_id,
                },
                contract,
                document_type,
            },
            override_document,
            &block_info,
            true,
            apply,
            transaction,
            &mut drive_operations,
            drive_version,
        )?;

        let fees = calculate_fee(None, Some(drive_operations), &block_info.epoch)?;

        Ok(fees)
    }
}