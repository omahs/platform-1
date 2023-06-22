use crate::data_contract::document_type::DocumentType;
use crate::data_contract::v0::DataContractV0;
use crate::data_contract::CreatedDataContract;
use crate::document::Document;
use crate::prelude::DataContract;
#[cfg(feature = "cbor")]
use crate::util::cbor_serializer::serializable_value_to_cbor;
use crate::version::FeatureVersion;
use crate::ProtocolError;
use platform_value::Identifier;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::data_contract::v0::created_data_contract::CreatedDataContractV0;

/// Reads a JSON file and converts it to serde_value.
pub fn json_document_to_json_value(
    path: impl AsRef<Path>,
) -> Result<serde_json::Value, ProtocolError> {
    let file = File::open(path.as_ref()).map_err(|_| {
        ProtocolError::FileNotFound(format!(
            "file not found at path {}",
            path.as_ref().to_str().unwrap()
        ))
    })?;

    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|_| ProtocolError::DecodingError("error decoding value from document".to_string()))
}

/// Reads a JSON file and converts it to serde_value.
pub fn json_document_to_platform_value(
    path: impl AsRef<Path>,
) -> Result<platform_value::Value, ProtocolError> {
    let file = File::open(path.as_ref()).map_err(|_| {
        ProtocolError::FileNotFound(format!(
            "file not found at path {}",
            path.as_ref().to_str().unwrap()
        ))
    })?;

    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|_| ProtocolError::DecodingError("error decoding value from document".to_string()))
}

/// Reads a JSON file and converts it to CBOR.
#[cfg(feature = "cbor")]
pub fn json_document_to_cbor(
    path: impl AsRef<Path>,
    protocol_version: Option<u32>,
) -> Result<Vec<u8>, ProtocolError> {
    let json = json_document_to_json_value(path)?;
    serializable_value_to_cbor(&json, protocol_version)
}

/// Reads a JSON file and converts it a contract.
pub fn json_document_to_contract(
    path: impl AsRef<Path>,
    version: FeatureVersion,
) -> Result<DataContract, ProtocolError> {
    let file = File::open(path.as_ref()).map_err(|_| {
        ProtocolError::FileNotFound(format!(
            "file not found at path {}",
            path.as_ref().to_str().unwrap()
        ))
    })?;

    let reader = BufReader::new(file);
    match version {
        0 => {
            let data_contract_v0: DataContractV0 =
                serde_json::from_reader(reader).map_err(|e| {
                    ProtocolError::DecodingError(format!(
                        "error decoding contract from document {e}"
                    ))
                })?;
            data_contract_v0.into()
        }
        version => Err(ProtocolError::UnknownVersionError(format!(
            "version {version} not known for json document to contract"
        ))),
    }
}

/// Reads a JSON file and converts it a contract.
pub fn json_document_to_created_contract(
    path: impl AsRef<Path>,
) -> Result<CreatedDataContract, ProtocolError> {
    let data_contract = json_document_to_contract(path)?;
    Ok(CreatedDataContractV0 {
        data_contract,
        entropy_used: Default::default(),
    }.into())
}

#[cfg(feature = "platform-value")]
/// Reads a JSON file and converts it a document.
pub fn json_document_to_contract_with_ids(
    path: impl AsRef<Path>,
    id: Option<Identifier>,
    owner_id: Option<Identifier>,
) -> Result<DataContract, ProtocolError> {
    let mut value = json_document_to_platform_value(path)?;
    if let Some(id) = id {
        value.set_value("$id", platform_value::Value::Identifier(id.into_buffer()))?;
    }
    if let Some(owner_id) = owner_id {
        value.set_value(
            "$ownerId",
            platform_value::Value::Identifier(owner_id.into_buffer()),
        )?;
    }
    DataContract::from_raw_object(value)
}

/// Reads a JSON file and converts it a document.
pub fn json_document_to_document(
    path: impl AsRef<Path>,
    owner_id: Option<Identifier>,
    document_type: &DocumentType,
) -> Result<Document, ProtocolError> {
    let mut value = json_document_to_platform_value(path)?;
    if let Some(owner_id) = owner_id {
        value.set_value(
            "$ownerId",
            platform_value::Value::Identifier(owner_id.into_buffer()),
        )?;
    }
    document_type.convert_value_to_document(value)
}

/// Makes sure the protocol version is correct given the version as a u8.
pub fn check_protocol_version_bytes(version_bytes: &[u8]) -> bool {
    if version_bytes.len() != 4 {
        false
    } else {
        let version_set_bytes: [u8; 4] = version_bytes
            .try_into()
            .expect("slice with incorrect length");
        let version = u32::from_be_bytes(version_set_bytes);
        check_protocol_version(version)
    }
}
