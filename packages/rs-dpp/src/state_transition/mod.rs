use derive_more::From;
use serde::{Deserialize, Serialize};

pub use abstract_state_transition::{
    state_transition_helpers, StateTransitionConvert, StateTransitionLike,
};
pub use abstract_state_transition_identity_signed::StateTransitionIdentitySignedV0;
use platform_value::{BinaryData, Value};
pub use state_transition_types::*;

// TODO unify the import paths ::object::state_transition::*
use crate::document::DocumentsBatchTransition;
use crate::prelude::Identifier;
use crate::serialization_traits::PlatformSerializable;
use bincode::{config, Decode, Encode};
use platform_serialization::{PlatformDeserialize, PlatformSerialize, PlatformSignable};

mod abstract_state_transition;
mod abstract_state_transition_identity_signed;
mod state_transition_facade;
mod state_transition_factory;
use crate::ProtocolError;
pub use state_transition_facade::*;
pub use state_transition_factory::*;

mod state_transition_types;
pub mod validation;

pub mod errors;
pub mod fee;
pub mod state_transition_execution_context;

pub mod apply_state_transition;
mod serialization;
mod state_transition_action;
mod state_transitions;
mod signing;

pub use state_transitions::*;

use crate::serialization_traits::{PlatformDeserializable, Signable};
use crate::util::hash;
use crate::version::FeatureVersion;
pub use state_transition_action::StateTransitionAction;
use crate::state_transition::data_contract_create_transition::DataContractCreateTransition;
use crate::state_transition::data_contract_update_transition::DataContractUpdateTransition;
use crate::state_transition::documents_batch_transition::DocumentsBatchTransition;
use crate::state_transition::identity_create_transition::IdentityCreateTransition;
use crate::state_transition::identity_credit_transfer_transition::IdentityCreditTransferTransition;
use crate::state_transition::identity_credit_withdrawal_transition::IdentityCreditWithdrawalTransition;
use crate::state_transition::identity_topup_transition::IdentityTopUpTransition;
use crate::state_transition::identity_update_transition::identity_update_transition::IdentityUpdateTransition;
macro_rules! call_method {
    ($state_transition:expr, $method:ident, $args:tt ) => {
        match $state_transition {
            StateTransition::DataContractCreate(st) => st.$method($args),
            StateTransition::DataContractUpdate(st) => st.$method($args),
            StateTransition::DocumentsBatch(st) => st.$method($args),
            StateTransition::IdentityCreate(st) => st.$method($args),
            StateTransition::IdentityTopUp(st) => st.$method($args),
            StateTransition::IdentityCreditWithdrawal(st) => st.$method($args),
            StateTransition::IdentityUpdate(st) => st.$method($args),
            StateTransition::IdentityCreditTransfer(st) => st.$method($args),
        }
    };
    ($state_transition:expr, $method:ident ) => {
        match $state_transition {
            StateTransition::DataContractCreate(st) => st.$method(),
            StateTransition::DataContractUpdate(st) => st.$method(),
            StateTransition::DocumentsBatch(st) => st.$method(),
            StateTransition::IdentityCreate(st) => st.$method(),
            StateTransition::IdentityTopUp(st) => st.$method(),
            StateTransition::IdentityCreditWithdrawal(st) => st.$method(),
            StateTransition::IdentityUpdate(st) => st.$method(),
            StateTransition::IdentityCreditTransfer(st) => st.$method(),
        }
    };
}

macro_rules! call_static_method {
    ($state_transition:expr, $method:ident ) => {
        match $state_transition {
            StateTransition::DataContractCreate(_) => DataContractCreateTransition::$method(),
            StateTransition::DataContractUpdate(_) => DataContractUpdateTransition::$method(),
            StateTransition::DocumentsBatch(_) => DocumentsBatchTransition::$method(),
            StateTransition::IdentityCreate(_) => IdentityCreateTransition::$method(),
            StateTransition::IdentityTopUp(_) => IdentityTopUpTransition::$method(),
            StateTransition::IdentityCreditWithdrawal(_) => {
                IdentityCreditWithdrawalTransition::$method()
            }
            StateTransition::IdentityUpdate(_) => IdentityUpdateTransition::$method(),
            StateTransition::IdentityCreditTransfer(_) => {
                IdentityCreditTransferTransition::$method()
            }
        }
    };
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    PlatformSerialize,
    PlatformDeserialize,
    PlatformSignable,
    From,
    PartialEq,
)]
#[platform_error_type(ProtocolError)]
#[platform_deserialize_limit(100000)]
#[platform_serialize_limit(100000)]
pub enum StateTransition {
    DataContractCreate(DataContractCreateTransition),
    DataContractUpdate(DataContractUpdateTransition),
    DocumentsBatch(DocumentsBatchTransition),
    IdentityCreate(IdentityCreateTransition),
    IdentityTopUp(IdentityTopUpTransition),
    IdentityCreditWithdrawal(IdentityCreditWithdrawalTransition),
    IdentityUpdate(IdentityUpdateTransition),
    IdentityCreditTransfer(IdentityCreditTransferTransition),
}

impl StateTransition {
    fn signature_property_paths(&self) -> Vec<&'static str> {
        call_static_method!(self, signature_property_paths)
    }

    fn identifiers_property_paths(&self) -> Vec<&'static str> {
        call_static_method!(self, identifiers_property_paths)
    }

    fn binary_property_paths(&self) -> Vec<&'static str> {
        call_static_method!(self, binary_property_paths)
    }

    pub fn get_owner_id(&self) -> &Identifier {
        call_method!(self, get_owner_id)
    }
}

impl StateTransitionConvert for StateTransition {
    fn hash(&self, skip_signature: bool) -> Result<Vec<u8>, ProtocolError> {
        if skip_signature {
            Ok(hash::hash_to_vec(self.signable_bytes()?))
        } else {
            Ok(hash::hash_to_vec(PlatformSerializable::serialize(self)?))
        }
    }

    #[cfg(feature = "cbor")]
    fn to_cbor_buffer(&self, _skip_signature: bool) -> Result<Vec<u8>, crate::ProtocolError> {
        call_method!(self, to_cbor_buffer, true)
    }

    #[cfg(feature = "json-object")]
    fn to_json(&self, skip_signature: bool) -> Result<serde_json::Value, crate::ProtocolError> {
        call_method!(self, to_json, skip_signature)
    }

    #[cfg(feature = "platform-value")]
    fn to_object(
        &self,
        skip_signature: bool,
    ) -> Result<platform_value::Value, crate::ProtocolError> {
        call_method!(self, to_object, skip_signature)
    }

    fn signature_property_paths() -> Vec<&'static str> {
        panic!("Static call is not supported")
    }

    fn identifiers_property_paths() -> Vec<&'static str> {
        panic!("Static call is not supported")
    }

    fn binary_property_paths() -> Vec<&'static str> {
        panic!("Static call is not supported")
    }

    #[cfg(feature = "platform-value")]
    fn to_cleaned_object(&self, skip_signature: bool) -> Result<Value, ProtocolError> {
        call_method!(self, to_cleaned_object, skip_signature)
    }
}

impl StateTransitionLike for StateTransition {
    fn state_transition_protocol_version(&self) -> FeatureVersion {
        call_method!(self, state_transition_protocol_version)
    }
    /// returns the type of State Transition
    fn state_transition_type(&self) -> StateTransitionType {
        call_method!(self, state_transition_type)
    }
    /// returns the signature as a byte-array
    fn signature(&self) -> &BinaryData {
        call_method!(self, signature)
    }

    /// set a new signature
    fn set_signature(&mut self, signature: BinaryData) {
        call_method!(self, set_signature, signature)
    }

    fn set_signature_bytes(&mut self, signature: Vec<u8>) {
        call_method!(self, set_signature_bytes, signature)
    }

    fn modified_data_ids(&self) -> Vec<crate::prelude::Identifier> {
        call_method!(self, modified_data_ids)
    }
}
