use crate::consensus::basic::identity::IdentityAssetLockTransactionOutputNotFoundError;
use crate::consensus::basic::BasicError;
use crate::consensus::ConsensusError;
use crate::identifier::Identifier;
use crate::identity::state_transition::identity_create_transition::IdentityCreateTransition;
use crate::identity::state_transition::identity_public_key_transitions::IdentityPublicKeyInCreation;
use crate::identity::{IdentityPublicKey, PartialIdentity};

use platform_value::Bytes36;
use serde::{Deserialize, Serialize};

pub const IDENTITY_CREATE_TRANSITION_ACTION_VERSION: u32 = 0;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityCreateTransitionAction {
    pub version: u32,
    pub public_keys: Vec<IdentityPublicKey>,
    pub initial_balance_amount: u64,
    pub identity_id: Identifier,
    pub asset_lock_outpoint: Bytes36,
}

impl From<IdentityCreateTransitionAction> for PartialIdentity {
    fn from(value: IdentityCreateTransitionAction) -> Self {
        let IdentityCreateTransitionAction {
            initial_balance_amount,
            identity_id,
            ..
        } = value;
        PartialIdentity {
            id: identity_id,
            loaded_public_keys: Default::default(), //no need to load public keys
            balance: Some(initial_balance_amount),
            revision: None,
            not_found_public_keys: Default::default(),
        }
    }
}

impl From<&IdentityCreateTransitionAction> for PartialIdentity {
    fn from(value: &IdentityCreateTransitionAction) -> Self {
        let IdentityCreateTransitionAction {
            initial_balance_amount,
            identity_id,
            ..
        } = value;
        PartialIdentity {
            id: *identity_id,
            loaded_public_keys: Default::default(), //no need to load public keys
            balance: Some(*initial_balance_amount),
            revision: None,
            not_found_public_keys: Default::default(),
        }
    }
}

impl IdentityCreateTransitionAction {
    pub fn current_version() -> u32 {
        IDENTITY_CREATE_TRANSITION_ACTION_VERSION
    }

    pub fn from(
        value: IdentityCreateTransition,
        initial_balance_amount: u64,
    ) -> Result<Self, ConsensusError> {
        let IdentityCreateTransition {
            public_keys,
            identity_id,
            asset_lock_proof,
            ..
        } = value;
        let asset_lock_outpoint = asset_lock_proof
            .out_point()
            .ok_or(ConsensusError::BasicError(
                BasicError::IdentityAssetLockTransactionOutputNotFoundError(
                    IdentityAssetLockTransactionOutputNotFoundError::new(
                        asset_lock_proof.instant_lock_output_index().unwrap(),
                    ),
                ),
            ))?
            .into();
        Ok(IdentityCreateTransitionAction {
            version: IDENTITY_CREATE_TRANSITION_ACTION_VERSION,
            public_keys: public_keys
                .into_iter()
                .map(IdentityPublicKeyInCreation::to_identity_public_key)
                .collect(),
            initial_balance_amount,
            identity_id,
            asset_lock_outpoint,
        })
    }

    pub fn from_borrowed(
        value: &IdentityCreateTransition,
        initial_balance_amount: u64,
    ) -> Result<Self, ConsensusError> {
        let IdentityCreateTransition {
            public_keys,
            identity_id,
            asset_lock_proof,
            ..
        } = value;
        let asset_lock_outpoint = asset_lock_proof
            .out_point()
            .ok_or(ConsensusError::BasicError(
                BasicError::IdentityAssetLockTransactionOutputNotFoundError(
                    IdentityAssetLockTransactionOutputNotFoundError::new(
                        asset_lock_proof.instant_lock_output_index().unwrap(),
                    ),
                ),
            ))?
            .into();
        Ok(IdentityCreateTransitionAction {
            version: IDENTITY_CREATE_TRANSITION_ACTION_VERSION,
            public_keys: public_keys
                .iter()
                .map(|key| key.clone().to_identity_public_key())
                .collect(),
            initial_balance_amount,
            identity_id: *identity_id,
            asset_lock_outpoint,
        })
    }
}
