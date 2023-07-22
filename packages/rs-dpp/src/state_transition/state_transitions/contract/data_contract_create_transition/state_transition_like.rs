use crate::identity::KeyID;
use crate::state_transition::data_contract_create_transition::v0::v0_methods::DataContractCreateTransitionV0Methods;
use crate::state_transition::data_contract_create_transition::DataContractCreateTransition;
use crate::state_transition::{StateTransitionLike, StateTransitionType};
use crate::version::FeatureVersion;
use platform_value::{BinaryData, Identifier};

impl StateTransitionLike for DataContractCreateTransition {
    /// Returns ID of the created contract
    fn modified_data_ids(&self) -> Vec<Identifier> {
        match self {
            DataContractCreateTransition::V0(transition) => transition.get_modified_data_ids(),
        }
    }

    fn state_transition_protocol_version(&self) -> FeatureVersion {
        match self {
            DataContractCreateTransition::V0(_) => 0,
        }
    }
    /// returns the type of State Transition
    fn state_transition_type(&self) -> StateTransitionType {
        match self {
            DataContractCreateTransition::V0(transition) => transition.state_transition_type(),
        }
    }
    /// returns the signature as a byte-array
    fn signature(&self) -> &BinaryData {
        match self {
            DataContractCreateTransition::V0(transition) => transition.signature(),
        }
    }
    /// set a new signature
    fn set_signature(&mut self, signature: BinaryData) {
        match self {
            DataContractCreateTransition::V0(transition) => transition.set_signature(signature),
        }
    }

    fn set_signature_bytes(&mut self, signature: Vec<u8>) {
        match self {
            DataContractCreateTransition::V0(transition) => {
                transition.set_signature_bytes(signature)
            }
        }
    }

    fn get_owner_id(&self) -> &Identifier {
        match self {
            DataContractCreateTransition::V0(transition) => transition.get_owner_id(),
        }
    }
}