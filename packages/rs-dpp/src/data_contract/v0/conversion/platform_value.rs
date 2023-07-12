use crate::data_contract::data_contract::DataContractV0;
use crate::data_contract::{property_names, DataContract, DocumentName, PropertyPath};
use crate::version::PlatformVersion;
use crate::ProtocolError;
use platform_value::btreemap_extensions::{BTreeValueMapHelper, BTreeValueRemoveFromMapHelper};
use platform_value::Value;
use std::collections::BTreeMap;
use crate::data_contract::conversion::platform_value_conversion::v0::DataContractValueConversionMethodsV0;
use crate::data_contract::property_names::SYSTEM_VERSION;
use serde_json::Value as JsonValue;

impl DataContractValueConversionMethodsV0 for DataContractV0 {
    fn to_object(&self) -> Result<Value, ProtocolError> {
        let mut value = platform_value::to_value(self).map_err(ProtocolError::ValueError)?;
        value.set_into_value(SYSTEM_VERSION, 0u16)?;
        Ok(value)
    }

    fn to_cleaned_object(&self) -> Result<Value, ProtocolError> {
        let mut value = self.to_object()?;
        if self.defs.is_none() {
            value.remove(property_names::DEFINITIONS)?;
        }
        Ok(value)
    }

    fn into_object(self) -> Result<Value, ProtocolError> {
        let mut value = platform_value::to_value(self).map_err(ProtocolError::ValueError)?;
        value.set_into_value(SYSTEM_VERSION, 0u16)?;
        Ok(value)
    }

    fn from_raw_object(
        raw_object: Value,
        platform_version: &PlatformVersion,
    ) -> Result<Self, ProtocolError> {
        let mut data_contract_map = raw_object
            .into_btree_string_map()
            .map_err(ProtocolError::ValueError)?;

        let id = data_contract_map
            .remove_identifier(property_names::ID)
            .map_err(ProtocolError::ValueError)?;

        let mutability = Self::get_contract_configuration_properties(&data_contract_map)?;
        let definition_references =
            DataContract::get_definitions(&data_contract_map, platform_version)?;
        let document_types = DataContract::get_document_types_from_contract(
            id,
            &data_contract_map,
            &definition_references,
            mutability.documents_keep_history_contract_default,
            mutability.documents_mutable_contract_default,
            platform_version,
        )?;

        let documents = data_contract_map
            .remove(property_names::DOCUMENTS)
            .map(|value| value.try_into_validating_btree_map_json())
            .transpose()?
            .unwrap_or_default();

        let mutability = Self::get_contract_configuration_properties(&data_contract_map)?;

        // Defs
        let defs =
            data_contract_map.get_optional_inner_str_json_value_map::<BTreeMap<_, _>>("$defs")?;

        let binary_properties = documents
            .iter()
            .map(|(doc_type, schema)| {
                Ok((
                    String::from(doc_type),
                    DataContract::get_binary_properties(schema, platform_version)?,
                ))
            })
            .collect::<Result<BTreeMap<DocumentName, BTreeMap<PropertyPath, JsonValue>>, ProtocolError>>()?;

        let data_contract = DataContractV0 {
            id,
            schema: data_contract_map
                .remove_string(property_names::SCHEMA)
                .map_err(ProtocolError::ValueError)?,
            version: data_contract_map
                .remove_integer(property_names::VERSION)
                .map_err(ProtocolError::ValueError)?,
            owner_id: data_contract_map
                .remove_identifier(property_names::OWNER_ID)
                .map_err(ProtocolError::ValueError)?,
            document_types,
            metadata: None,
            config: mutability,
            documents,
            defs,
            binary_properties,
        };

        Ok(data_contract)
    }
}