use crate::consensus::state::state_error::StateError;
use crate::consensus::ConsensusError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::prelude::Identifier;

use bincode::{Decode, Encode};

#[derive(Error, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
#[error("Insufficient identity ${identity_id} balance ${balance}")]
pub struct IdentityInsufficientBalanceError {
    /*

    DO NOT CHANGE ORDER OF FIELDS WITHOUT INTRODUCING OF NEW VERSION

    */
    pub identity_id: Identifier,
    pub balance: u64,
}

impl IdentityInsufficientBalanceError {
    pub fn new(identity_id: Identifier, balance: u64) -> Self {
        Self {
            identity_id,
            balance,
        }
    }

    pub fn identity_id(&self) -> &Identifier {
        &self.identity_id
    }

    pub fn balance(&self) -> u64 {
        self.balance
    }
}
impl From<IdentityInsufficientBalanceError> for ConsensusError {
    fn from(err: IdentityInsufficientBalanceError) -> Self {
        Self::StateError(StateError::IdentityInsufficientBalanceError(err))
    }
}
