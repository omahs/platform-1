use crate::buffer::Buffer;
use dpp::consensus::basic::identity::InvalidInstantAssetLockProofSignatureError;
use dpp::consensus::codes::ErrorWithCode;
use dpp::consensus::ConsensusError;
use dpp::serialization_traits::PlatformSerializable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=InvalidInstantAssetLockProofSignatureError)]
pub struct InvalidInstantAssetLockProofSignatureErrorWasm {
    inner: InvalidInstantAssetLockProofSignatureError,
}

impl From<&InvalidInstantAssetLockProofSignatureError>
    for InvalidInstantAssetLockProofSignatureErrorWasm
{
    fn from(e: &InvalidInstantAssetLockProofSignatureError) -> Self {
        Self { inner: e.clone() }
    }
}

#[wasm_bindgen(js_class=InvalidInstantAssetLockProofSignatureError)]
impl InvalidInstantAssetLockProofSignatureErrorWasm {
    #[wasm_bindgen(js_name=getCode)]
    pub fn get_code(&self) -> u32 {
        ConsensusError::from(self.inner.clone()).code()
    }

    #[wasm_bindgen(getter)]
    pub fn message(&self) -> String {
        self.inner.to_string()
    }

    #[wasm_bindgen(js_name=serialize)]
    pub fn serialize(&self) -> Result<Buffer, JsError> {
        let bytes = ConsensusError::from(self.inner.clone())
            .serialize()
            .map_err(JsError::from)?;

        Ok(Buffer::from_bytes(bytes.as_slice()))
    }
}
