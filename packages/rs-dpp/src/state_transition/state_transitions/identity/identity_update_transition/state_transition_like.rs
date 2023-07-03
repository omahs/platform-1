use platform_value::{BinaryData, Identifier};
use crate::state_transition::identity_update_transition::IdentityUpdateTransition;
use crate::state_transition::{StateTransitionLike, StateTransitionType};
use crate::state_transition::identity_update_transition::v0::v0_methods::IdentityUpdateTransitionV0Methods;
use crate::version::FeatureVersion;

impl StateTransitionLike for IdentityUpdateTransition {
    /// Returns ID of the updated contract
    fn modified_data_ids(&self) -> Vec<Identifier> {
        match self {
            IdentityUpdateTransition::V0(transition) => transition.get_modified_data_ids(),
        }
    }

    fn state_transition_protocol_version(&self) -> FeatureVersion {
        match self {
            IdentityUpdateTransition::V0(_) => 0,
        }
    }
    /// returns the type of State Transition
    fn state_transition_type(&self) -> StateTransitionType {
        match self {
            IdentityUpdateTransition::V0(transition) => transition.state_transition_type(),
        }
    }
    /// returns the signature as a byte-array
    fn signature(&self) -> &BinaryData {
        match self {
            IdentityUpdateTransition::V0(transition) => transition.signature(),
        }
    }
    /// set a new signature
    fn set_signature(&mut self, signature: BinaryData) {
        match self {
            IdentityUpdateTransition::V0(transition) => transition.set_signature(signature),
        }
    }

    fn set_signature_bytes(&mut self, signature: Vec<u8>) {
        match self {
            IdentityUpdateTransition::V0(transition) => {
                transition.set_signature_bytes(signature)
            }
        }
    }

    fn get_owner_id(&self) -> &Identifier {
        match self {
            IdentityUpdateTransition::V0(transition) => {
                transition.get_owner_id()
            }
        }
    }
}