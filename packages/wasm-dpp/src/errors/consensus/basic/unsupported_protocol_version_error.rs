use crate::buffer::Buffer;
use dpp::consensus::basic::UnsupportedProtocolVersionError;
use dpp::consensus::codes::ErrorWithCode;
use dpp::consensus::ConsensusError;
use dpp::serialization_traits::PlatformSerializable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name=UnsupportedProtocolVersionError)]
pub struct UnsupportedProtocolVersionErrorWasm {
    inner: UnsupportedProtocolVersionError,
}

impl From<&UnsupportedProtocolVersionError> for UnsupportedProtocolVersionErrorWasm {
    fn from(e: &UnsupportedProtocolVersionError) -> Self {
        Self { inner: e.clone() }
    }
}

#[wasm_bindgen(js_class=UnsupportedProtocolVersionError)]
impl UnsupportedProtocolVersionErrorWasm {
    #[wasm_bindgen(js_name=getParsedProtocolVersion)]
    pub fn parsed_protocol_version(&self) -> u32 {
        self.inner.parsed_protocol_version()
    }

    #[wasm_bindgen(js_name=getLatestVersion)]
    pub fn latest_version(&self) -> u32 {
        self.inner.latest_version()
    }

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
