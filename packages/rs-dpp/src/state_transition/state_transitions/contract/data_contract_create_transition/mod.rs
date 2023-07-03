mod action;
mod v0;
mod v0_action;
mod fields;
#[cfg(feature = "json-object")]
mod json_conversion;
#[cfg(feature = "platform-value")]
mod value_conversion;
mod state_transition_like;
mod v0_methods;
mod identity_signed;
mod serialize;

use fields::*;

use crate::data_contract::property_names::ENTROPY;

use crate::data_contract::DataContract;
use crate::document::document_transition::document_base_transition::JsonValue;
use crate::identity::KeyID;
use crate::serialization_traits::PlatformDeserializable;
use crate::serialization_traits::{PlatformSerializable, Signable};
use crate::state_transition::{StateTransitionFieldTypes, StateTransitionLike, StateTransitionType};
use crate::version::{PlatformVersion};
use crate::{ProtocolError};
pub use action::DataContractCreateTransitionAction;
use bincode::{config, Decode, Encode};
use derive_more::From;
use platform_serialization::{PlatformDeserialize, PlatformSerialize};
use platform_versioning::{PlatformSerdeVersioned, PlatformVersioned};
use platform_value::{BinaryData, Bytes32, Identifier, Value};
use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use std::fmt;
pub use v0::*;
pub use v0_action::*;
use crate::data_contract::state_transition::property_names::{SIGNATURE, SIGNATURE_PUBLIC_KEY_ID};

pub type DataContractCreateTransitionLatest = DataContractCreateTransitionV0;

#[derive(Debug, Clone, PlatformDeserialize, PlatformSerialize, PlatformSerdeVersioned, PlatformVersioned, Encode, Decode, From, PartialEq)]
#[platform_error_type(ProtocolError)]
#[platform_version_path(state_transitions.contract_create_state_transition)]
pub enum DataContractCreateTransition {
    V0(DataContractCreateTransitionV0),
}


impl From<DataContract> for DataContractCreateTransition {
    fn from(value: DataContract) -> Self {
        DataContractCreateTransitionV0::from(value).into()
    }
}

impl Signable for DataContractCreateTransition {
    fn signable_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        match self {
            DataContractCreateTransition::V0(transition) => transition.signable_bytes(),
        }
    }
}

impl StateTransitionFieldTypes for DataContractCreateTransition {
    fn signature_property_paths() -> Vec<&'static str> {
        vec![SIGNATURE, SIGNATURE_PUBLIC_KEY_ID]
    }

    fn identifiers_property_paths() -> Vec<&'static str> {
        vec![]
    }

    fn binary_property_paths() -> Vec<&'static str> {
        vec![SIGNATURE, ENTROPY]
    }
}

impl DataContractCreateTransition {
    pub fn state_transition_version(&self) -> u16 {
        match self {
            DataContractCreateTransition::V0(_) => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data_contract::state_transition::property_names::TRANSITION_TYPE;
    use crate::data_contract::CreatedDataContract;
    use integer_encoding::VarInt;
    use platform_value::Bytes32;

    use crate::state_transition::{StateTransitionType, StateTransitionValueConvert};
    use crate::tests::fixtures::get_data_contract_fixture;
    use crate::util::json_value::JsonValueExt;
    use crate::version::LATEST_PLATFORM_VERSION;
    use crate::{Convertible, version};
    use crate::state_transition::state_transitions::common_fields::property_names;
    use super::*;

    pub(crate) struct TestData {
        pub(crate) state_transition: DataContractCreateTransition,
        pub(crate) created_data_contract: CreatedDataContract,
    }

    pub(crate) fn get_test_data() -> TestData {
        let created_data_contract = get_data_contract_fixture(None);

        let state_transition = DataContractCreateTransition::from_raw_object(Value::from([
            (
                STATE_TRANSITION_PROTOCOL_VERSION,
                LATEST_PLATFORM_VERSION
                    .state_transitions
                    .contract_create_state_transition
                    .default_current_version
                    .into(),
            ),
            (
                property_names::ENTROPY,
                created_data_contract.entropy_used().into(),
            ),
            (
                DATA_CONTRACT,
                created_data_contract.data_contract().to_object().unwrap(),
            ),
        ]))
        .expect("state transition should be created without errors");

        TestData {
            created_data_contract,
            state_transition,
        }
    }

    #[test]
    fn should_return_protocol_version() {
        let data = get_test_data();
        assert_eq!(
            LATEST_PLATFORM_VERSION
                .state_transitions
                .contract_create_state_transition
                .default_current_version,
            data.state_transition.state_transition_protocol_version()
        )
    }

    #[test]
    fn should_return_transition_type() {
        let data = get_test_data();
        assert_eq!(
            StateTransitionType::DataContractCreate,
            data.state_transition.state_transition_type()
        );
    }

    #[test]
    fn should_return_data_contract() {
        let data = get_test_data();

        assert_eq!(
            data.state_transition
                .data_contract()
                .to_json_object()
                .expect("conversion to object shouldn't fail"),
            data.created_data_contract
                .data_contract
                .to_json_object()
                .expect("conversion to object shouldn't fail")
        );
    }


    #[test]
    fn should_return_serialized_state_transition_to_buffer() {
        let data = get_test_data();
        let state_transition_bytes = data
            .state_transition
            .to_cbor_buffer(false)
            .expect("state transition should be converted to buffer");
        let (protocol_version, _) =
            u32::decode_var(state_transition_bytes.as_ref()).expect("expected to decode");
        assert_eq!(version::LATEST_VERSION, protocol_version)
    }

    #[test]
    fn should_return_owner_id() {
        let data = get_test_data();
        assert_eq!(
            &data.created_data_contract.data_contract.owner_id,
            data.state_transition.get_owner_id()
        );
    }

    #[test]
    fn is_data_contract_state_transition() {
        let data = get_test_data();
        assert!(data.state_transition.is_data_contract_state_transition());
        assert!(!data.state_transition.is_document_state_transition());
        assert!(!data.state_transition.is_identity_state_transition());
    }
}
