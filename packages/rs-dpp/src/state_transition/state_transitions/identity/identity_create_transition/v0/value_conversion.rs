use std::collections::BTreeMap;
use std::convert::{TryFrom, TryInto};

use platform_value::btreemap_extensions::{
    BTreeValueMapHelper, BTreeValueRemoveFromMapHelper, BTreeValueRemoveInnerValueFromMapHelper,
};
use platform_value::{BinaryData, Bytes32, IntegerReplacementType, ReplacementType, Value};
use serde::{Deserialize, Serialize};

use crate::{
    data_contract::DataContract,
    identity::KeyID,
    prelude::Identifier,
    state_transition::{StateTransitionFieldTypes, StateTransitionLike, StateTransitionType},
    Convertible, NonConsensusError, ProtocolError,
};

use crate::prelude::AssetLockProof;
use crate::serialization_traits::{PlatformDeserializable, Signable};
use crate::state_transition::identity_create_transition::fields::*;
use crate::state_transition::identity_create_transition::v0::v0_methods::IdentityCreateTransitionV0Methods;
use crate::state_transition::identity_create_transition::v0::IdentityCreateTransitionV0;
use crate::state_transition::identity_public_key_transitions::IdentityPublicKeyInCreation;
use crate::state_transition::StateTransitionValueConvert;
use bincode::{config, Decode, Encode};

impl StateTransitionValueConvert for IdentityCreateTransitionV0 {
    fn from_raw_object(raw_object: Value) -> Result<Self, ProtocolError> {
        let mut state_transition = Self::default();

        let mut transition_map = raw_object
            .into_btree_string_map()
            .map_err(ProtocolError::ValueError)?;
        if let Some(keys_value_array) = transition_map
            .remove_optional_inner_value_array::<Vec<_>>(PUBLIC_KEYS)
            .map_err(ProtocolError::ValueError)?
        {
            let keys = keys_value_array
                .into_iter()
                .map(|val| val.try_into().map_err(ProtocolError::ValueError))
                .collect::<Result<Vec<IdentityPublicKeyInCreation>, ProtocolError>>()?;
            state_transition.set_public_keys(keys);
        }

        if let Some(proof) = transition_map.get(ASSET_LOCK_PROOF) {
            state_transition.set_asset_lock_proof(AssetLockProof::try_from(proof)?)?;
        }

        if let Some(signature) = transition_map.get_optional_binary_data(SIGNATURE)? {
            state_transition.set_signature(signature);
        }

        Ok(state_transition)
    }

    fn clean_value(value: &mut Value) -> Result<(), ProtocolError> {
        value.replace_at_paths(IDENTIFIER_FIELDS, ReplacementType::Identifier)?;
        value.replace_at_paths(BINARY_FIELDS, ReplacementType::BinaryBytes)?;
        value.replace_integer_type_at_paths(U32_FIELDS, IntegerReplacementType::U32)?;
        Ok(())
    }

    fn from_value_map(
        mut raw_data_contract_create_transition: BTreeMap<String, Value>,
    ) -> Result<Self, ProtocolError> {
        todo()
    }

    fn to_object(&self, skip_signature: bool) -> Result<Value, ProtocolError> {
        let mut value: Value = platform_value::to_value(self)?;

        if skip_signature {
            value
                .remove_values_matching_paths(Self::signature_property_paths())
                .map_err(ProtocolError::ValueError)?;
        }

        let mut public_keys: Vec<Value> = vec![];
        for key in self.public_keys.iter() {
            public_keys.push(key.to_raw_object(skip_signature)?);
        }

        value.insert(PUBLIC_KEYS.to_owned(), Value::Array(public_keys))?;

        Ok(value)
    }

    fn to_cleaned_object(&self, skip_signature: bool) -> Result<Value, ProtocolError> {
        let mut value: Value = platform_value::to_value(self)?;

        if skip_signature {
            value
                .remove_values_matching_paths(Self::signature_property_paths())
                .map_err(ProtocolError::ValueError)?;
        }

        let mut public_keys: Vec<Value> = vec![];
        for key in self.public_keys.iter() {
            public_keys.push(key.to_raw_cleaned_object(skip_signature)?);
        }

        value.insert(PUBLIC_KEYS.to_owned(), Value::Array(public_keys))?;

        Ok(value)
    }
}
