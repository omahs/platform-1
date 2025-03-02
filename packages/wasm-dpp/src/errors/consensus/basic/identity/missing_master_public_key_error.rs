use crate::buffer::Buffer;
use dpp::consensus::basic::identity::MissingMasterPublicKeyError;
use dpp::consensus::codes::ErrorWithCode;
use dpp::consensus::ConsensusError;
use dpp::serialization_traits::PlatformSerializable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=MissingMasterPublicKeyError)]
pub struct MissingMasterPublicKeyErrorWasm {
    inner: MissingMasterPublicKeyError,
}

impl From<&MissingMasterPublicKeyError> for MissingMasterPublicKeyErrorWasm {
    fn from(e: &MissingMasterPublicKeyError) -> Self {
        Self { inner: e.clone() }
    }
}

#[wasm_bindgen(js_class=MissingMasterPublicKeyError)]
impl MissingMasterPublicKeyErrorWasm {
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
