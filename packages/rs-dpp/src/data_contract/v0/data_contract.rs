use bincode::de::{BorrowDecoder, Decoder};
use bincode::enc::Encoder;
use bincode::error::{DecodeError, EncodeError};
use bincode::{BorrowDecode, Decode, Encode};
use std::collections::{BTreeMap, HashSet};
use std::convert::{TryFrom, TryInto};

use itertools::{Either, Itertools};
use platform_serialization::PlatformSerialize;
use platform_value::btreemap_extensions::{BTreeValueMapHelper, BTreeValueRemoveFromMapHelper};
use platform_value::Identifier;
use platform_value::{ReplacementType, Value, ValueMapHelper};
use serde::de::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::consensus::basic::document::InvalidDocumentTypeError;
use crate::data_contract::contract_config::{
    ContractConfigV0, DEFAULT_CONTRACT_CAN_BE_DELETED, DEFAULT_CONTRACT_DOCUMENTS_KEEPS_HISTORY,
    DEFAULT_CONTRACT_DOCUMENT_MUTABILITY, DEFAULT_CONTRACT_KEEPS_HISTORY,
    DEFAULT_CONTRACT_MUTABILITY,
};
use crate::data_contract::{
    contract_config, DataContract, DefinitionName, DocumentName, JsonSchema, PropertyPath,
};

use crate::data_contract::document_type::v0::DocumentTypeV0;
use crate::data_contract::document_type::{DocumentType, DocumentTypeRef};
#[cfg(feature = "cbor")]
use crate::util::cbor_serializer;
use crate::{errors::ProtocolError, metadata::Metadata, util::hash::hash_to_vec};
use crate::{identifier, Convertible};
use platform_value::string_encoding::Encoding;

use crate::data_contract::errors::DataContractError;
use crate::data_contract::property_names::{SYSTEM_VERSION, VERSION};
use crate::data_contract::serialized_version::DataContractSerializationFormat;
use crate::util::deserializer::ProtocolVersion;
use crate::version::PlatformVersion;

use super::super::property_names;

pub const DATA_CONTRACT_SCHEMA_URI_V0: &str =
    "https://schema.dash.org/dpp-0-4-0/meta/data-contract";

pub const DATA_CONTRACT_IDENTIFIER_FIELDS_V0: [&str; 2] =
    [property_names::ID, property_names::OWNER_ID];
pub const DATA_CONTRACT_BINARY_FIELDS_V0: [&str; 1] = [property_names::ENTROPY];

/// `DataContractV0` represents a data contract in a decentralized platform.
///
/// It contains information about the contract, such as its protocol version, unique identifier,
/// schema, version, and owner identifier. The struct also includes details about the document
/// types, metadata, configuration, and document schemas associated with the contract.
///
/// Additionally, `DataContractV0` holds definitions for JSON schemas, entropy, and binary properties
/// of the documents.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "DataContractV0Inner")]
#[serde(rename_all = "camelCase")]
pub struct DataContractV0 {
    /// A unique identifier for the data contract.
    /// This field must always present in all versions.
    #[serde(rename = "$id")]
    pub id: Identifier,

    /// A reference to the JSON schema that defines the contract.
    #[serde(rename = "$schema")]
    pub schema: String,

    /// The version of this data contract.
    pub version: u32,

    /// The identifier of the contract owner.
    pub owner_id: Identifier,

    /// A mapping of document names to their corresponding document types.
    #[serde(skip)]
    pub document_types: BTreeMap<DocumentName, DocumentType>,

    /// Optional metadata associated with the contract.
    #[serde(skip)]
    pub metadata: Option<Metadata>,

    /// Internal configuration for the contract.
    #[serde(skip)]
    pub config: ContractConfigV0,

    /// A mapping of document names to their corresponding JSON schemas.
    pub documents: BTreeMap<DocumentName, JsonSchema>,

    /// Optional mapping of definition names to their corresponding JSON schemas.
    #[serde(rename = "$defs", default)]
    pub defs: Option<BTreeMap<DefinitionName, JsonSchema>>,

    /// A nested mapping of document names and property paths to their binary values.
    #[serde(skip)]
    pub binary_properties: BTreeMap<DocumentName, BTreeMap<PropertyPath, JsonValue>>,
}

impl Decode for DataContractV0 {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Self, DecodeError> {
        let inner = DataContractSerializationFormat::decode(decoder)?;
        inner
            .try_into()
            .map_err(|e: ProtocolError| DecodeError::custom(e.to_string()))
    }
}

impl<'a> BorrowDecode<'a> for DataContractV0 {
    fn borrow_decode<D: BorrowDecoder<'a>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let inner = DataContractV0Inner::decode(decoder)?;
        inner
            .try_into()
            .map_err(|e: ProtocolError| DecodeError::custom(e.to_string()))
    }
}

// Standalone default_protocol_version function
fn default_protocol_version() -> u32 {
    1
}

#[derive(Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct DataContractV0Inner {
    /// A unique identifier for the data contract.
    #[serde(rename = "$version")]
    pub version0: u16,

    /// A unique identifier for the data contract.
    #[serde(rename = "$id")]
    pub id: Identifier,

    /// Internal configuration for the contract.
    #[serde(default)]
    pub config: ContractConfigV0,

    /// A reference to the JSON schema that defines the contract.
    #[serde(rename = "$schema")]
    pub schema: String,

    /// The version of this data contract.
    pub version: u32,

    /// The identifier of the contract owner.
    pub owner_id: Identifier,

    /// A mapping of document names to their corresponding JSON values.
    pub documents: BTreeMap<DocumentName, Value>,

    /// Optional mapping of definition names to their corresponding JSON values.
    #[serde(rename = "$defs", default)]
    pub defs: Option<BTreeMap<DefinitionName, Value>>,
}

impl From<DataContractV0> for DataContractV0Inner {
    fn from(value: DataContractV0) -> Self {
        let DataContractV0 {
            id,
            config,
            schema,
            version,
            owner_id,
            documents,
            defs,
            ..
        } = value;
        DataContractV0Inner {
            version0: 0,
            id,
            config,
            schema,
            version,
            owner_id,
            documents: documents
                .into_iter()
                .map(|(key, value)| (key, value.into()))
                .collect(),
            defs: defs.map(|defs| {
                defs.into_iter()
                    .map(|(key, value)| (key, value.into()))
                    .collect()
            }),
        }
    }
}

impl DataContractV0 {
    fn try_from_inner(value: DataContractV0Inner, platform_version: &PlatformVersion) -> Result<Self, ProtocolError> {
        let DataContractV0Inner {
            id,
            config,
            schema,
            version,
            owner_id,
            documents,
            defs,
            ..
        } = value;

        let document_types = DataContract::get_document_types_from_value_array(
            id,
            &documents
                .iter()
                .map(|(key, value)| (key.as_str(), value))
                .collect(),
            &defs
                .as_ref()
                .map(|defs| {
                    defs.iter()
                        .map(|(key, value)| Ok((key.clone(), value)))
                        .collect::<Result<BTreeMap<String, &Value>, ProtocolError>>()
                })
                .transpose()?
                .unwrap_or_default(),
            config.documents_keep_history_contract_default,
            config.documents_mutable_contract_default,
            platform_version,
        )?;

        let binary_properties = documents
            .iter()
            .map(|(doc_type, schema)| Ok((String::from(doc_type), DataContract::get_binary_properties(&schema.clone().try_into()?, platform_version))))
            .collect::<Result<BTreeMap<DocumentName, BTreeMap<PropertyPath, JsonValue>>, ProtocolError>>()?;

        let data_contract = DataContractV0 {
            id,
            schema,
            version,
            owner_id,
            document_types,
            metadata: None,
            config,
            documents: documents
                .into_iter()
                .map(|(key, value)| Ok((key, value.try_into()?)))
                .collect::<Result<BTreeMap<DocumentName, JsonSchema>, ProtocolError>>()?,
            defs: defs
                .map(|defs| {
                    defs.into_iter()
                        .map(|(key, value)| Ok((key, value.try_into()?)))
                        .collect::<Result<BTreeMap<DefinitionName, JsonSchema>, ProtocolError>>()
                })
                .transpose()?,
            binary_properties,
        };

        Ok(data_contract)
    }
}

impl DataContractV0 {
    /// Retrieve contract configuration properties.
    ///
    /// This method takes a BTreeMap representing a contract and retrieves
    /// the configuration properties based on the values found in the map.
    ///
    /// The process of retrieving contract configuration properties is versioned,
    /// and the version is determined by the platform version parameter.
    /// If the version is not supported, an error is returned.
    ///
    /// # Parameters
    ///
    /// * `contract`: BTreeMap representing the contract.
    /// * `platform_version`: The platform version being used.
    ///
    /// # Returns
    ///
    /// * `Result<ContractConfig, ProtocolError>`: On success, a ContractConfig.
    ///   On failure, a ProtocolError.
    pub(in crate::data_contract::v0) fn get_contract_configuration_properties(
        contract: &BTreeMap<String, Value>,
    ) -> Result<ContractConfigV0, ProtocolError> {
        let keeps_history = contract
            .get_optional_bool(contract_config::property::KEEPS_HISTORY)?
            .unwrap_or(DEFAULT_CONTRACT_KEEPS_HISTORY);
        let can_be_deleted = contract
            .get_optional_bool(contract_config::property::CAN_BE_DELETED)?
            .unwrap_or(DEFAULT_CONTRACT_CAN_BE_DELETED);

        let readonly = contract
            .get_optional_bool(contract_config::property::READONLY)?
            .unwrap_or(!DEFAULT_CONTRACT_MUTABILITY);

        let documents_keep_history_contract_default = contract
            .get_optional_bool(contract_config::property::DOCUMENTS_KEEP_HISTORY_CONTRACT_DEFAULT)?
            .unwrap_or(DEFAULT_CONTRACT_DOCUMENTS_KEEPS_HISTORY);

        let documents_mutable_contract_default = contract
            .get_optional_bool(contract_config::property::DOCUMENTS_MUTABLE_CONTRACT_DEFAULT)?
            .unwrap_or(DEFAULT_CONTRACT_DOCUMENT_MUTABILITY);

        Ok(ContractConfigV0 {
            can_be_deleted,
            readonly,
            keeps_history,
            documents_keep_history_contract_default,
            documents_mutable_contract_default,
        })
    }
}

//
// #[cfg(feature = "json-object")]
// impl TryFrom<JsonValue> for DataContractV0 {
//     type Error = ProtocolError;
//     fn try_from(v: JsonValue) -> Result<Self, Self::Error> {
//         DataContractV0::from_json_object(v)
//     }
// }
//
// #[cfg(feature = "platform-value")]
// impl TryFrom<Value> for DataContractV0 {
//     type Error = ProtocolError;
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         DataContractV0::from_raw_object(value)
//     }
// }
//
// impl TryFrom<DataContractV0> for Value {
//     type Error = ProtocolError;
//
//     fn try_from(value: DataContractV0) -> Result<Self, Self::Error> {
//         value.into_object()
//     }
// }
//
// impl TryFrom<&DataContractV0> for Value {
//     type Error = ProtocolError;
//
//     fn try_from(value: &DataContractV0) -> Result<Self, Self::Error> {
//         value.to_object()
//     }
// }
//
// #[cfg(feature = "platform-value")]
// impl TryFrom<&str> for DataContractV0 {
//     type Error = ProtocolError;
//     fn try_from(v: &str) -> Result<Self, Self::Error> {
//         let data_contract: DataContractV0 = serde_json::from_str(v)?;
//         //todo: there's a better to do this, find it
//         let value = data_contract.to_object()?;
//         DataContractV0::from_raw_object(value)
//     }
// }
//
// impl<'a> TryFrom<&'a [u8]> for DataContractV0 {
//     type Error = ProtocolError;
//
//     fn try_from(_v: &[u8]) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }
//
// impl TryFrom<Vec<u8>> for DataContractV0 {
//     type Error = ProtocolError;
//
//     fn try_from(_v: Vec<u8>) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

#[cfg(test)]
mod test {
    use anyhow::Result;
    use integer_encoding::VarInt;

    use crate::tests::{fixtures::get_data_contract_fixture, utils::*};

    use super::*;

    fn init() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .try_init();
    }

    #[test]
    #[cfg(feature = "cbor")]
    fn conversion_to_cbor_buffer_from_cbor_buffer() {
        init();
        let data_contract = get_data_contract_fixture(None).data_contract;

        let data_contract_bytes = data_contract
            .to_cbor_buffer()
            .expect("data contract should be converted into the bytes");
        let data_contract_restored = DataContractV0::from_cbor_buffer(data_contract_bytes)
            .expect("data contract should be created from bytes");

        assert_eq!(
            data_contract.data_contract_protocol_version,
            data_contract_restored.data_contract_protocol_version
        );
        assert_eq!(data_contract.schema, data_contract_restored.schema);
        assert_eq!(data_contract.version, data_contract_restored.version);
        assert_eq!(data_contract.id, data_contract_restored.id);
        assert_eq!(data_contract.owner_id, data_contract_restored.owner_id);
        assert_eq!(
            data_contract.binary_properties,
            data_contract_restored.binary_properties
        );
        assert_eq!(data_contract.documents, data_contract_restored.documents);
        assert_eq!(
            data_contract.document_types,
            data_contract_restored.document_types
        );
    }

    #[test]
    #[cfg(feature = "cbor")]
    fn conversion_to_cbor_buffer_from_cbor_buffer_high_version() {
        init();
        let mut data_contract = get_data_contract_fixture(None).data_contract;
        data_contract.data_contract_protocol_version = 10000;

        let data_contract_bytes = data_contract
            .to_cbor_buffer()
            .expect("data contract should be converted into the bytes");

        let data_contract_restored = DataContractV0::from_cbor_buffer(data_contract_bytes)
            .expect("data contract should be created from bytes");

        assert_eq!(
            data_contract.data_contract_protocol_version,
            data_contract_restored.data_contract_protocol_version
        );
        assert_eq!(data_contract.schema, data_contract_restored.schema);
        assert_eq!(data_contract.version, data_contract_restored.version);
        assert_eq!(data_contract.id, data_contract_restored.id);
        assert_eq!(data_contract.owner_id, data_contract_restored.owner_id);
        assert_eq!(
            data_contract.binary_properties,
            data_contract_restored.binary_properties
        );
        assert_eq!(data_contract.documents, data_contract_restored.documents);
        assert_eq!(
            data_contract.document_types,
            data_contract_restored.document_types
        );
    }

    #[test]
    fn conversion_to_cbor_buffer_from_cbor_buffer_too_high_version() {
        init();
        let data_contract = get_data_contract_fixture(None).data_contract;

        let data_contract_bytes = data_contract
            .to_cbor_buffer()
            .expect("data contract should be converted into the bytes");

        let mut high_protocol_version_bytes = u64::MAX.encode_var_vec();

        let (_, offset) = u32::decode_var(&data_contract_bytes)
            .ok_or(ProtocolError::DecodingError(
                "contract cbor could not decode protocol version".to_string(),
            ))
            .expect("expected to decode protocol version");
        let (_, contract_cbor_bytes) = data_contract_bytes.split_at(offset);

        high_protocol_version_bytes.extend_from_slice(contract_cbor_bytes);

        let data_contract_restored = DataContractV0::from_cbor_buffer(&high_protocol_version_bytes)
            .expect("data contract should be created from bytes");

        assert_eq!(
            u32::MAX,
            data_contract_restored.data_contract_protocol_version
        );
        assert_eq!(data_contract.schema, data_contract_restored.schema);
        assert_eq!(data_contract.version, data_contract_restored.version);
        assert_eq!(data_contract.id, data_contract_restored.id);
        assert_eq!(data_contract.owner_id, data_contract_restored.owner_id);
        assert_eq!(
            data_contract.binary_properties,
            data_contract_restored.binary_properties
        );
        assert_eq!(data_contract.documents, data_contract_restored.documents);
        assert_eq!(
            data_contract.document_types,
            data_contract_restored.document_types
        );
    }

    #[test]
    fn conversion_from_json() -> Result<()> {
        init();

        let string_contract = get_data_from_file("src/tests/payloads/contract_example.json")?;
        let contract = DataContractV0::try_from(string_contract.as_str())?;
        assert_eq!(contract.data_contract_protocol_version, 0);
        assert_eq!(
            contract.schema,
            "https://schema.dash.org/dpp-0-4-0/meta/data-contract"
        );
        assert_eq!(contract.version, 5);
        assert_eq!(
            contract.id.to_string(Encoding::Base58),
            "AoDzJxWSb1gUi2dSmvFeUFpSsjZQRJaqCpn7vCLkwwJj"
        );
        assert_eq!(
            contract.documents["note"]["properties"]["message"]["type"],
            "string"
        );
        assert!(contract.is_document_defined("note"));

        Ok(())
    }

    #[test]
    fn conversion_to_json() -> Result<()> {
        init();

        let mut string_contract = get_data_from_file("src/tests/payloads/contract_example.json")?;
        string_contract.retain(|c| !c.is_whitespace());

        let contract = DataContractV0::try_from(string_contract.as_str())?;
        let serialized_contract = serde_json::to_string(&contract.to_json()?)?;

        // they will be out of order so won't be exactly the same
        assert_eq!(serialized_contract, string_contract);
        Ok(())
    }

    #[test]
    fn conversion_to_object() -> Result<()> {
        let string_contract = get_data_from_file("src/tests/payloads/contract_example.json")?;
        let data_contract: DataContractV0 = serde_json::from_str(&string_contract)?;

        let raw_data_contract = data_contract.to_json_object()?;
        for path in DATA_CONTRACT_IDENTIFIER_FIELDS_V0 {
            assert!(raw_data_contract
                .get(path)
                .expect("the path should exist")
                .is_array())
        }
        Ok(())
    }

    #[test]
    fn conversion_from_object() -> Result<()> {
        init();

        let string_contract = get_data_from_file("src/tests/payloads/contract_example.json")?;
        let raw_contract: JsonValue = serde_json::from_str(&string_contract)?;

        for path in DATA_CONTRACT_IDENTIFIER_FIELDS_V0 {
            raw_contract.get(path).expect("the path should exist");
        }

        let data_contract_from_raw = DataContractV0::try_from(raw_contract)?;
        assert_eq!(data_contract_from_raw.data_contract_protocol_version, 0);
        assert_eq!(
            data_contract_from_raw.schema,
            "https://schema.dash.org/dpp-0-4-0/meta/data-contract"
        );
        assert_eq!(data_contract_from_raw.version, 5);
        assert_eq!(
            data_contract_from_raw.id.to_string(Encoding::Base58),
            "AoDzJxWSb1gUi2dSmvFeUFpSsjZQRJaqCpn7vCLkwwJj"
        );
        assert_eq!(
            data_contract_from_raw.documents["note"]["properties"]["message"]["type"],
            "string"
        );

        Ok(())
    }

    fn get_data_contract_cbor_bytes() -> Vec<u8> {
        let data_contract_cbor_hex = "01a56324696458208efef7338c0d34b2e408411b9473d724cbf9b675ca72b3126f7f8e7deb42ae516724736368656d61783468747470733a2f2f736368656d612e646173682e6f72672f6470702d302d342d302f6d6574612f646174612d636f6e7472616374676f776e657249645820962088aa3812bb3386d0c9130edbde51e4be17bb2d10031d4147c8597facee256776657273696f6e0169646f63756d656e7473a76b756e697175654461746573a56474797065666f626a65637467696e646963657382a3646e616d6566696e6465783166756e69717565f56a70726f7065727469657382a16a2463726561746564417463617363a16a2475706461746564417463617363a2646e616d6566696e646578326a70726f7065727469657381a16a2475706461746564417463617363687265717569726564836966697273744e616d656a246372656174656441746a247570646174656441746a70726f70657274696573a2686c6173744e616d65a1647479706566737472696e676966697273744e616d65a1647479706566737472696e67746164646974696f6e616c50726f70657274696573f46c6e696365446f63756d656e74a46474797065666f626a656374687265717569726564816a246372656174656441746a70726f70657274696573a1646e616d65a1647479706566737472696e67746164646974696f6e616c50726f70657274696573f46e6e6f54696d65446f63756d656e74a36474797065666f626a6563746a70726f70657274696573a1646e616d65a1647479706566737472696e67746164646974696f6e616c50726f70657274696573f46e707265747479446f63756d656e74a46474797065666f626a65637468726571756972656482686c6173744e616d656a247570646174656441746a70726f70657274696573a1686c6173744e616d65a1647479706566737472696e67746164646974696f6e616c50726f70657274696573f46e7769746842797465417272617973a56474797065666f626a65637467696e646963657381a2646e616d6566696e646578316a70726f7065727469657381a16e6279746541727261794669656c6463617363687265717569726564816e6279746541727261794669656c646a70726f70657274696573a26e6279746541727261794669656c64a36474797065656172726179686d61784974656d731069627974654172726179f56f6964656e7469666965724669656c64a56474797065656172726179686d61784974656d731820686d696e4974656d73182069627974654172726179f570636f6e74656e744d656469615479706578216170706c69636174696f6e2f782e646173682e6470702e6964656e746966696572746164646974696f6e616c50726f70657274696573f46f696e6465786564446f63756d656e74a56474797065666f626a65637467696e646963657386a3646e616d6566696e6465783166756e69717565f56a70726f7065727469657382a168246f776e6572496463617363a16966697273744e616d656464657363a3646e616d6566696e6465783266756e69717565f56a70726f7065727469657382a168246f776e6572496463617363a1686c6173744e616d656464657363a2646e616d6566696e646578336a70726f7065727469657381a1686c6173744e616d6563617363a2646e616d6566696e646578346a70726f7065727469657382a16a2463726561746564417463617363a16a2475706461746564417463617363a2646e616d6566696e646578356a70726f7065727469657381a16a2475706461746564417463617363a2646e616d6566696e646578366a70726f7065727469657381a16a2463726561746564417463617363687265717569726564846966697273744e616d656a246372656174656441746a24757064617465644174686c6173744e616d656a70726f70657274696573a2686c6173744e616d65a2647479706566737472696e67696d61784c656e677468183f6966697273744e616d65a2647479706566737472696e67696d61784c656e677468183f746164646974696f6e616c50726f70657274696573f4781d6f7074696f6e616c556e69717565496e6465786564446f63756d656e74a56474797065666f626a65637467696e646963657383a3646e616d6566696e6465783166756e69717565f56a70726f7065727469657381a16966697273744e616d656464657363a3646e616d6566696e6465783266756e69717565f56a70726f7065727469657383a168246f776e6572496463617363a16966697273744e616d6563617363a1686c6173744e616d6563617363a3646e616d6566696e6465783366756e69717565f56a70726f7065727469657382a167636f756e74727963617363a1646369747963617363687265717569726564826966697273744e616d65686c6173744e616d656a70726f70657274696573a46463697479a2647479706566737472696e67696d61784c656e677468183f67636f756e747279a2647479706566737472696e67696d61784c656e677468183f686c6173744e616d65a2647479706566737472696e67696d61784c656e677468183f6966697273744e616d65a2647479706566737472696e67696d61784c656e677468183f746164646974696f6e616c50726f70657274696573f4";
        hex::decode(data_contract_cbor_hex).unwrap()
    }

    #[test]
    fn deserialize_dpp_cbor() {
        let data_contract_cbor = get_data_contract_cbor_bytes();

        let data_contract = DataContractV0::from_cbor_buffer(data_contract_cbor).unwrap();

        assert_eq!(data_contract.version, 1);
        assert_eq!(data_contract.data_contract_protocol_version, 1);
        assert_eq!(
            data_contract.schema,
            "https://schema.dash.org/dpp-0-4-0/meta/data-contract"
        );
        assert_eq!(
            data_contract.owner_id,
            Identifier::new([
                150, 32, 136, 170, 56, 18, 187, 51, 134, 208, 201, 19, 14, 219, 222, 81, 228, 190,
                23, 187, 45, 16, 3, 29, 65, 71, 200, 89, 127, 172, 238, 37
            ])
        );
        assert_eq!(
            data_contract.id,
            Identifier::new([
                142, 254, 247, 51, 140, 13, 52, 178, 228, 8, 65, 27, 148, 115, 215, 36, 203, 249,
                182, 117, 202, 114, 179, 18, 111, 127, 142, 125, 235, 66, 174, 81
            ])
        );
    }

    #[test]
    fn serialize_deterministically_serialize_to_cbor() {
        let data_contract_cbor = get_data_contract_cbor_bytes();

        let data_contract = DataContractV0::from_cbor_buffer(&data_contract_cbor).unwrap();

        let serialized = data_contract.to_cbor_buffer().unwrap();

        assert_eq!(hex::encode(data_contract_cbor), hex::encode(serialized));
    }

    #[test]
    fn serialize_deterministically_serialize_to_bincode() {
        let data_contract_cbor = get_data_contract_cbor_bytes();

        let data_contract = DataContractV0::from_cbor_buffer(&data_contract_cbor).unwrap();

        let serialized = data_contract.to_cbor_buffer().unwrap();

        assert_eq!(hex::encode(data_contract_cbor), hex::encode(serialized));
    }
}