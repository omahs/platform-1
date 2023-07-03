use serde_json::Number;
use crate::ProtocolError;
use crate::state_transition::{JsonSerializationOptions, StateTransitionJsonConvert};
use crate::state_transition::identity_create_transition::{IdentityCreateTransition};
use crate::state_transition::documents_batch_transition::document_base_transition::JsonValue;
use crate::state_transition::state_transitions::identity_create_transition::fields::*;

impl StateTransitionJsonConvert for IdentityCreateTransition {
    fn to_json(&self, options: JsonSerializationOptions) -> Result<JsonValue, ProtocolError> {
        match self {
            IdentityCreateTransition::V0(transition) => {
                let mut value = transition.to_json(options)?;
                let map_value = value.as_object_mut().expect("expected an object");
                map_value.insert(STATE_TRANSITION_PROTOCOL_VERSION.to_string(), JsonValue::Number(Number::from(0)))?;
                Ok(value)
            }
        }
    }
}