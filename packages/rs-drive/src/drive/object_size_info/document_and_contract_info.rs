use dpp::data_contract::document_type::DocumentTypeRef;
use dpp::data_contract::DataContract;
use crate::drive::object_size_info::OwnedDocumentInfo;

/// Document and contract info
#[derive(Clone, Debug)]
pub struct DocumentAndContractInfo<'a> {
    /// Document info
    pub owned_document_info: OwnedDocumentInfo<'a>,
    /// Contract
    pub contract: &'a Contract,
    /// Document type
    pub document_type: &'a DocumentTypeRef<'a>,
}