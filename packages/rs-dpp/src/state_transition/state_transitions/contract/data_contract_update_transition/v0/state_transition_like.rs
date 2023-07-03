use platform_serialization::{PlatformDeserialize, PlatformSerialize};

use platform_value::btreemap_extensions::BTreeValueRemoveFromMapHelper;
use platform_value::{BinaryData, Bytes32, IntegerReplacementType, ReplacementType, Value};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::{Convertible, data_contract::DataContract, identity::KeyID, NonConsensusError, prelude::Identifier, ProtocolError, state_transition::{
    StateTransitionFieldTypes, StateTransitionLike,
    StateTransitionType,
}};

use crate::serialization_traits::{PlatformDeserializable, Signable};
use bincode::{config, Decode, Encode};
use crate::identity::PartialIdentity;
use crate::identity::signer::Signer;
use crate::state_transition::data_contract_update_transition::DataContractUpdateTransitionV0;
use crate::state_transition::StateTransition;
use crate::state_transition::StateTransitionType::{DataContractUpdate};
use crate::version::FeatureVersion;

impl StateTransitionLike for DataContractUpdateTransitionV0 {
    /// Returns ID of the created contract
    fn modified_data_ids(&self) -> Vec<Identifier> {
        vec![self.data_contract.id()]
    }

    fn state_transition_protocol_version(&self) -> FeatureVersion {
        0
    }
    /// returns the type of State Transition
    fn state_transition_type(&self) -> StateTransitionType {
        DataContractUpdate
    }
    /// returns the signature as a byte-array
    fn signature(&self) -> &BinaryData {
        &self.signature
    }
    /// set a new signature
    fn set_signature(&mut self, signature: BinaryData) {
        self.signature = signature
    }

    fn set_signature_bytes(&mut self, signature: Vec<u8>) {
        self.signature = BinaryData::new(signature)
    }

    /// Get owner ID
    fn get_owner_id(&self) -> &Identifier {
        &self.data_contract.owner_id()
    }
}