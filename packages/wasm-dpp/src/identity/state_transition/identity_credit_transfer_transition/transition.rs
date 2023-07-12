use crate::utils::{ToSerdeJSONExt, WithJsError};

use std::convert::TryInto;
use std::default::Default;

use wasm_bindgen::prelude::*;

use crate::bls_adapter::{BlsAdapter, JsBlsAdapter};
use crate::identifier::IdentifierWrapper;
use crate::{buffer::Buffer, errors::RustConversionError, with_js_error, IdentityPublicKeyWasm};
use dpp::identifier::Identifier;
use dpp::identity::state_transition::identity_credit_transfer_transition::IdentityCreditTransferTransition;
use dpp::identity::KeyType;
use dpp::platform_value::string_encoding::Encoding;
use dpp::platform_value::{string_encoding, BinaryData};
use dpp::serialization_traits::PlatformSerializable;
use dpp::state_transition::StateTransitionLike;
use dpp::state_transition::{StateTransition, StateTransitionIdentitySignedV0};

#[wasm_bindgen(js_name=IdentityCreditTransferTransition)]
#[derive(Clone)]
pub struct IdentityCreditTransferTransitionWasm(IdentityCreditTransferTransition);

impl From<IdentityCreditTransferTransition> for IdentityCreditTransferTransitionWasm {
    fn from(v: IdentityCreditTransferTransition) -> Self {
        IdentityCreditTransferTransitionWasm(v)
    }
}

impl From<IdentityCreditTransferTransitionWasm> for IdentityCreditTransferTransition {
    fn from(v: IdentityCreditTransferTransitionWasm) -> Self {
        v.0
    }
}

#[wasm_bindgen(js_class = IdentityCreditTransferTransition)]
impl IdentityCreditTransferTransitionWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(raw_parameters: JsValue) -> Result<IdentityCreditTransferTransitionWasm, JsValue> {
        let raw_state_transition = raw_parameters.with_serde_to_platform_value()?;

        let identity_credit_transfer_transition =
            IdentityCreditTransferTransition::from_raw_object(raw_state_transition)
                .map_err(|e| RustConversionError::Error(e.to_string()).to_js_value())?;

        Ok(identity_credit_transfer_transition.into())
    }

    #[wasm_bindgen(js_name=getType)]
    pub fn get_type(&self) -> u8 {
        IdentityCreditTransferTransition::get_type() as u8
    }

    #[wasm_bindgen(getter, js_name=identityId)]
    pub fn identity_id(&self) -> IdentifierWrapper {
        self.get_identity_id()
    }

    #[wasm_bindgen(getter, js_name=recipientId)]
    pub fn recipient_id(&self) -> IdentifierWrapper {
        self.get_recipient_id()
    }

    #[wasm_bindgen(getter, js_name=amount)]
    pub fn amount(&self) -> u64 {
        self.0.get_amount()
    }

    #[wasm_bindgen(js_name=getIdentityId)]
    pub fn get_identity_id(&self) -> IdentifierWrapper {
        (*self.0.get_identity_id()).into()
    }

    #[wasm_bindgen(js_name=getRecipientId)]
    pub fn get_recipient_id(&self) -> IdentifierWrapper {
        (*self.0.get_recipient_id()).into()
    }

    #[wasm_bindgen(js_name=getAmount)]
    pub fn get_amount(&self) -> f64 {
        self.0.get_amount() as f64
    }

    #[wasm_bindgen(js_name=setAmount)]
    pub fn set_amount(&mut self, amount: f64) {
        self.0.set_amount(amount as u64);
    }

    #[wasm_bindgen(js_name=toObject)]
    pub fn to_object(&self, options: JsValue) -> Result<JsValue, JsValue> {
        let opts: super::to_object::ToObjectOptions = if options.is_object() {
            with_js_error!(serde_wasm_bindgen::from_value(options))?
        } else {
            Default::default()
        };

        let object = super::to_object::to_object_struct(&self.0, opts);
        let js_object = js_sys::Object::new();

        js_sys::Reflect::set(
            &js_object,
            &"type".to_owned().into(),
            &object.transition_type.into(),
        )?;

        js_sys::Reflect::set(
            &js_object,
            &"protocolVersion".to_owned().into(),
            &object.protocol_version.into(),
        )?;

        if let Some(signature) = object.signature {
            let signature_value: JsValue = if signature.is_empty() {
                JsValue::undefined()
            } else {
                Buffer::from_bytes(signature.as_slice()).into()
            };

            js_sys::Reflect::set(&js_object, &"signature".to_owned().into(), &signature_value)?;

            if let Some(signature_public_key_id) = object.signature_public_key_id {
                js_sys::Reflect::set(
                    &js_object,
                    &"signaturePublicKeyId".to_owned().into(),
                    &JsValue::from(signature_public_key_id),
                )?;
            } else {
                js_sys::Reflect::set(
                    &js_object,
                    &"signaturePublicKeyId".to_owned().into(),
                    &JsValue::undefined(),
                )?;
            }
        }

        js_sys::Reflect::set(
            &js_object,
            &"identityId".to_owned().into(),
            &Buffer::from_bytes(object.identity_id.to_buffer().as_slice()),
        )?;

        js_sys::Reflect::set(
            &js_object,
            &"recipientId".to_owned().into(),
            &Buffer::from_bytes(object.recipient_id.to_buffer().as_slice()),
        )?;

        js_sys::Reflect::set(
            &js_object,
            &"amount".to_owned().into(),
            &JsValue::from_f64(object.amount as f64),
        )?;

        Ok(js_object.into())
    }

    #[wasm_bindgen(js_name=toBuffer)]
    pub fn to_buffer(&self) -> Result<Buffer, JsValue> {
        let bytes = PlatformSerializable::serialize(&StateTransition::IdentityCreditTransfer(
            self.0.clone(),
        ))
        .with_js_error()?;
        Ok(Buffer::from_bytes(&bytes))
    }

    #[wasm_bindgen(js_name=toJSON)]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        let object = super::to_object::to_object_struct(&self.0, Default::default());
        let js_object = js_sys::Object::new();

        js_sys::Reflect::set(
            &js_object,
            &"type".to_owned().into(),
            &object.transition_type.into(),
        )?;

        js_sys::Reflect::set(
            &js_object,
            &"protocolVersion".to_owned().into(),
            &object.protocol_version.into(),
        )?;

        if let Some(signature) = object.signature {
            let signature_value: JsValue = if signature.is_empty() {
                JsValue::undefined()
            } else {
                string_encoding::encode(signature.as_slice(), Encoding::Base64).into()
            };

            js_sys::Reflect::set(&js_object, &"signature".to_owned().into(), &signature_value)?;

            if let Some(signature_public_key_id) = object.signature_public_key_id {
                js_sys::Reflect::set(
                    &js_object,
                    &"signaturePublicKeyId".to_owned().into(),
                    &signature_public_key_id.into(),
                )?;
            } else {
                js_sys::Reflect::set(
                    &js_object,
                    &"signaturePublicKeyId".to_owned().into(),
                    &JsValue::undefined(),
                )?;
            }
        }

        let identity_id = object.identity_id.to_string(Encoding::Base58);

        js_sys::Reflect::set(
            &js_object,
            &"identityId".to_owned().into(),
            &identity_id.into(),
        )?;

        let recipient_id = object.recipient_id.to_string(Encoding::Base58);

        js_sys::Reflect::set(
            &js_object,
            &"recipientId".to_owned().into(),
            &recipient_id.into(),
        )?;

        js_sys::Reflect::set(
            &js_object,
            &"amount".to_owned().into(),
            &JsValue::from_f64(object.amount as f64),
        )?;

        Ok(js_object.into())
    }

    #[wasm_bindgen(js_name=getModifiedDataIds)]
    pub fn get_modified_data_ids(&self) -> Vec<JsValue> {
        let ids = self.0.get_modified_data_ids();

        ids.into_iter()
            .map(|id| <IdentifierWrapper as std::convert::From<Identifier>>::from(id).into())
            .collect()
    }

    #[wasm_bindgen(js_name=isDataContractStateTransition)]
    pub fn is_data_contract_state_transition(&self) -> bool {
        self.0.is_data_contract_state_transition()
    }

    #[wasm_bindgen(js_name=isDocumentStateTransition)]
    pub fn is_document_state_transition(&self) -> bool {
        self.0.is_document_state_transition()
    }

    #[wasm_bindgen(js_name=isIdentityStateTransition)]
    pub fn is_identity_state_transition(&self) -> bool {
        self.0.is_identity_state_transition()
    }

    #[wasm_bindgen(js_name=signByPrivateKey)]
    pub fn sign_by_private_key(
        &mut self,
        private_key: Vec<u8>,
        key_type: u8,
        bls: Option<JsBlsAdapter>,
    ) -> Result<(), JsValue> {
        let key_type = key_type
            .try_into()
            .map_err(|e: anyhow::Error| e.to_string())?;

        if bls.is_none() && key_type == KeyType::BLS12_381 {
            return Err(JsError::new(
                format!("BLS adapter is required for BLS key type '{}'", key_type).as_str(),
            )
            .into());
        }

        let bls_adapter = if let Some(adapter) = bls {
            BlsAdapter(adapter)
        } else {
            BlsAdapter(JsValue::undefined().into())
        };

        self.0
            .sign_by_private_key(private_key.as_slice(), key_type, &bls_adapter)
            .with_js_error()
    }

    #[wasm_bindgen(js_name=getSignature)]
    pub fn get_signature(&self) -> Buffer {
        Buffer::from_bytes(self.0.get_signature().as_slice())
    }

    #[wasm_bindgen(js_name=setSignature)]
    pub fn set_signature(&mut self, signature: Option<Vec<u8>>) {
        self.0.signature = BinaryData::new(signature.unwrap_or(vec![]))
    }

    #[wasm_bindgen]
    pub fn sign(
        &mut self,
        identity_public_key: &IdentityPublicKeyWasm,
        private_key: Vec<u8>,
        bls: JsBlsAdapter,
    ) -> Result<(), JsValue> {
        let bls_adapter = BlsAdapter(bls);
        self.0
            .sign(
                &identity_public_key.to_owned().into(),
                &private_key,
                &bls_adapter,
            )
            .with_js_error()
    }
}