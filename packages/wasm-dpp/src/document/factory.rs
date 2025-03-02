use anyhow::anyhow;
use dpp::platform_value::ReplacementType;
use dpp::{
    document::{
        document_factory::{DocumentFactory, FactoryOptions},
        document_transition::Action,
        extended_document,
        fetch_and_validate_data_contract::DataContractFetcherAndValidator,
    },
    ProtocolError,
};
use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use crate::document::errors::InvalidActionNameError;
use dpp::platform_value::btreemap_extensions::BTreeValueMapReplacementPathHelper;
use dpp::prelude::ExtendedDocument;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::entropy_generator::ExternalEntropyGenerator;
use crate::{
    identifier::identifier_from_js_value,
    state_repository::{ExternalStateRepositoryLike, ExternalStateRepositoryLikeWrapper},
    utils::{IntoWasm, ToSerdeJSONExt, WithJsError},
    DataContractWasm, DocumentsBatchTransitionWasm, ExtendedDocumentWasm,
};

use super::validator::DocumentValidatorWasm;

#[wasm_bindgen(js_name=DocumentTransitions)]
#[derive(Debug, Default)]
pub struct DocumentTransitions {
    create: Vec<ExtendedDocumentWasm>,
    replace: Vec<ExtendedDocumentWasm>,
    delete: Vec<ExtendedDocumentWasm>,
}

#[wasm_bindgen(js_class=DocumentTransitions)]
impl DocumentTransitions {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen(js_name = "addTransitionCreate")]
    pub fn add_transition_create(&mut self, transition: ExtendedDocumentWasm) {
        self.create.push(transition)
    }

    #[wasm_bindgen(js_name = "addTransitionReplace")]
    pub fn add_transition_replace(&mut self, transition: ExtendedDocumentWasm) {
        self.replace.push(transition)
    }

    #[wasm_bindgen(js_name = "addTransitionDelete")]
    pub fn add_transition_delete(&mut self, transition: ExtendedDocumentWasm) {
        self.delete.push(transition)
    }
}

#[wasm_bindgen(js_name = DocumentFactory)]
pub struct DocumentFactoryWASM(DocumentFactory<ExternalStateRepositoryLikeWrapper>);

impl DocumentFactoryWASM {
    pub(crate) fn new_with_state_repository_wrapper(
        protocol_version: u32,
        document_validator: DocumentValidatorWasm,
        entropy_generator: ExternalEntropyGenerator,
        state_repository: ExternalStateRepositoryLikeWrapper,
    ) -> Self {
        let factory = DocumentFactory::new_with_entropy_generator(
            protocol_version,
            document_validator.into(),
            DataContractFetcherAndValidator::new(Arc::new(state_repository)),
            Box::new(entropy_generator),
        );

        DocumentFactoryWASM(factory)
    }
}

#[wasm_bindgen(js_class=DocumentFactory)]
impl DocumentFactoryWASM {
    #[wasm_bindgen(constructor)]
    pub fn new(
        protocol_version: u32,
        document_validator: DocumentValidatorWasm,
        state_repository: ExternalStateRepositoryLike,
        external_entropy_generator_arg: Option<ExternalEntropyGenerator>,
    ) -> DocumentFactoryWASM {
        let factory = if let Some(external_entropy_generator) = external_entropy_generator_arg {
            DocumentFactory::new_with_entropy_generator(
                protocol_version,
                document_validator.into(),
                DataContractFetcherAndValidator::new(Arc::new(
                    ExternalStateRepositoryLikeWrapper::new(state_repository),
                )),
                Box::new(external_entropy_generator),
            )
        } else {
            DocumentFactory::new(
                protocol_version,
                document_validator.into(),
                DataContractFetcherAndValidator::new(Arc::new(
                    ExternalStateRepositoryLikeWrapper::new(state_repository),
                )),
            )
        };

        DocumentFactoryWASM(factory)
    }

    #[wasm_bindgen]
    pub fn create(
        &self,
        data_contract: &DataContractWasm,
        js_owner_id: &JsValue,
        document_type: &str,
        data: &JsValue,
    ) -> Result<ExtendedDocumentWasm, JsValue> {
        let owner_id = identifier_from_js_value(js_owner_id)?;
        let dynamic_data = data.with_serde_to_platform_value()?;
        let document = self
            .0
            .create_extended_document_for_state_transition(
                data_contract.to_owned().into(),
                owner_id,
                document_type.to_string(),
                dynamic_data,
            )
            .with_js_error()?;

        Ok(document.into())
    }

    #[wasm_bindgen(js_name=createStateTransition)]
    pub fn create_state_transition(
        &self,
        documents: &JsValue,
    ) -> Result<DocumentsBatchTransitionWasm, JsValue> {
        let documents_by_action = extract_documents_by_action(documents)?;
        let batch_transition = self
            .0
            .create_state_transition(documents_by_action)
            .with_js_error()?;

        Ok(batch_transition.into())
    }

    #[wasm_bindgen(js_name=createFromObject)]
    pub async fn create_from_object(
        &self,
        raw_document_js: JsValue,
        options: JsValue,
    ) -> Result<ExtendedDocumentWasm, JsValue> {
        let mut raw_document = raw_document_js.with_serde_to_platform_value()?;
        let options: FactoryOptions = if !options.is_undefined() && options.is_object() {
            let raw_options = options.with_serde_to_json_value()?;
            serde_json::from_value(raw_options).with_js_error()?
        } else {
            Default::default()
        };
        raw_document
            .replace_at_paths(
                extended_document::IDENTIFIER_FIELDS,
                ReplacementType::Identifier,
            )
            .map_err(ProtocolError::ValueError)
            .with_js_error()?;

        let mut document = self
            .0
            .create_from_object(raw_document, options)
            .await
            .with_js_error()?;
        let (identifier_paths, binary_paths): (Vec<_>, Vec<_>) = document
            .get_identifiers_and_binary_paths_owned()
            .with_js_error()?;
        // When data contract is available, replace remaining dynamic paths
        let document_data = document.properties_as_mut();
        document_data
            .replace_at_paths(identifier_paths, ReplacementType::Identifier)
            .map_err(ProtocolError::ValueError)
            .with_js_error()?;
        document_data
            .replace_at_paths(binary_paths, ReplacementType::BinaryBytes)
            .map_err(ProtocolError::ValueError)
            .with_js_error()?;
        Ok(document.into())
    }

    #[wasm_bindgen(js_name=createFromBuffer)]
    pub async fn create_from_buffer(
        &self,
        buffer: Vec<u8>,
        options: &JsValue,
    ) -> Result<ExtendedDocumentWasm, JsValue> {
        let options: FactoryOptions = if !options.is_undefined() && options.is_object() {
            let raw_options = options.with_serde_to_json_value()?;
            serde_json::from_value(raw_options).with_js_error()?
        } else {
            Default::default()
        };

        let document = self
            .0
            .create_from_buffer(buffer, options)
            .await
            .with_js_error()?;

        Ok(document.into())
    }

    #[wasm_bindgen(js_name=createExtendedDocumentFromDocumentBuffer)]
    pub fn create_extended_from_document_buffer(
        &self,
        buffer: Vec<u8>,
        document_type: &str,
        data_contract: &DataContractWasm,
    ) -> Result<ExtendedDocumentWasm, JsValue> {
        self.0
            .create_extended_from_document_buffer(
                buffer.as_slice(),
                document_type,
                &data_contract.to_owned().into(),
            )
            .map(|document| document.into())
            .with_js_error()
    }
}

fn extract_documents_by_action(
    documents: &JsValue,
) -> Result<HashMap<Action, Vec<ExtendedDocument>>, JsValue> {
    check_actions(documents)?;
    let mut documents_by_action: HashMap<Action, Vec<ExtendedDocument>> = Default::default();

    let documents_create = extract_documents_of_action(documents, "create").with_js_error()?;
    let documents_replace = extract_documents_of_action(documents, "replace").with_js_error()?;
    let documents_delete = extract_documents_of_action(documents, "delete").with_js_error()?;

    documents_by_action.insert(Action::Create, documents_create);
    documents_by_action.insert(Action::Replace, documents_replace);
    documents_by_action.insert(Action::Delete, documents_delete);

    Ok(documents_by_action)
}

fn check_actions(documents: &JsValue) -> Result<(), JsValue> {
    if !documents.is_object() {
        return Err(anyhow!("Expected documents to be an object")).with_js_error();
    }

    let documents_object = js_sys::Object::from(documents.clone());

    let actions: js_sys::Array = js_sys::Object::keys(&documents_object);

    for action in actions.iter() {
        let action_string: String = action
            .as_string()
            .ok_or_else(|| anyhow!("Expected all keys to be strings"))
            .with_js_error()?;
        Action::try_from(action_string)
            .map_err(|_| InvalidActionNameError::new(vec![action.clone()]))?;
    }

    Ok(())
}

fn extract_documents_of_action(
    documents: &JsValue,
    action: &str,
) -> Result<Vec<ExtendedDocument>, anyhow::Error> {
    let mut extracted_documents: Vec<ExtendedDocument> = vec![];
    let documents_with_action =
        js_sys::Reflect::get(documents, &action.to_string().into()).unwrap_or(JsValue::NULL);
    if documents_with_action.is_null() || documents_with_action.is_undefined() {
        return Ok(extracted_documents);
    }
    let documents_array = js_sys::Array::try_from(documents_with_action)
        .map_err(|e| anyhow!("property '{}' isn't an array: {}", action, e))?;

    for js_document in documents_array.iter() {
        let document: ExtendedDocument = js_document
            .to_wasm::<ExtendedDocumentWasm>("ExtendedDocument")
            .map_err(|e| {
                anyhow!(
                    "Element in '{}' isn't an Extended Document instance: {:#?}",
                    action,
                    e
                )
            })?
            .clone()
            .into();
        extracted_documents.push(document)
    }

    Ok(extracted_documents)
}
