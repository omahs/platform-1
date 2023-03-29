use std::convert::TryInto;
use std::sync::Arc;

use anyhow::Result;
use dashcore::{consensus, BlockHeader};
use platform_value::platform_value;

use crate::consensus::signature::IdentityNotFoundError;

use crate::contracts::withdrawals_contract;
use crate::document::{generate_document_id, Document};
use crate::identity::state_transition::identity_credit_withdrawal_transition::{
    IdentityCreditWithdrawalTransitionAction, Pooling,
};
use crate::validation::ValidationResult;
use crate::{
    consensus::basic::identity::IdentityInsufficientBalanceError,
    identity::state_transition::identity_credit_withdrawal_transition::IdentityCreditWithdrawalTransition,
    state_repository::StateRepositoryLike, state_transition::StateTransitionLike,
    NonConsensusError, ProtocolError, StateError,
};

pub struct IdentityCreditWithdrawalTransitionValidator<SR>
where
    SR: StateRepositoryLike,
{
    state_repository: Arc<SR>,
}

impl<SR> IdentityCreditWithdrawalTransitionValidator<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new(state_repository: Arc<SR>) -> Self {
        Self { state_repository }
    }

    pub async fn validate_identity_credit_withdrawal_transition_state(
        &self,
        state_transition: &IdentityCreditWithdrawalTransition,
    ) -> Result<ValidationResult<IdentityCreditWithdrawalTransitionAction>, ProtocolError> {
        let mut result = ValidationResult::default();

        // TODO: Use fetchIdentityBalance
        let maybe_existing_identity = self
            .state_repository
            .fetch_identity(
                &state_transition.identity_id,
                Some(state_transition.get_execution_context()),
            )
            .await?
            .map(TryInto::try_into)
            .transpose()
            .map_err(Into::into)
            .map_err(|e| {
                NonConsensusError::StateRepositoryFetchError(format!(
                    "state repository fetch identity for credit withdrawal verification error: {}",
                    e.to_string()
                ))
            })?;

        let Some(existing_identity) = maybe_existing_identity else {
            let err = IdentityNotFoundError::new(state_transition.identity_id);

            result.add_error(err);

            return Ok(result);
        };

        if existing_identity.get_balance() < state_transition.amount {
            let err = IdentityInsufficientBalanceError {
                identity_id: state_transition.identity_id,
                balance: existing_identity.balance,
            };

            result.add_error(err);

            return Ok(result);
        }

        // Check revision
        if existing_identity.get_revision() != (state_transition.get_revision() - 1) {
            result.add_error(StateError::InvalidIdentityRevisionError {
                identity_id: existing_identity.get_id().to_owned(),
                current_revision: existing_identity.get_revision(),
            });

            return Ok(result);
        }

        let document_id = generate_document_id::generate_document_id(
            &withdrawals_contract::CONTRACT_ID,
            &state_transition.identity_id,
            withdrawals_contract::document_types::WITHDRAWAL,
            state_transition.output_script.as_bytes(),
        );

        let latest_platform_block_header_bytes: Vec<u8> = self
            .state_repository
            .fetch_latest_platform_block_header()
            .await?;

        let latest_platform_block_header: BlockHeader =
            consensus::deserialize(&latest_platform_block_header_bytes)
                .map_err(ProtocolError::DashCoreError)?;

        let document_created_at_millis: i64 = latest_platform_block_header.time as i64 * 1000i64;

        let document_data = platform_value!({
            withdrawals_contract::property_names::AMOUNT: state_transition.amount,
            withdrawals_contract::property_names::CORE_FEE_PER_BYTE: state_transition.core_fee_per_byte,
            withdrawals_contract::property_names::POOLING: Pooling::Never,
            withdrawals_contract::property_names::OUTPUT_SCRIPT: state_transition.output_script.as_bytes(),
            withdrawals_contract::property_names::STATUS: withdrawals_contract::WithdrawalStatus::QUEUED,
            withdrawals_contract::property_names::CREATED_AT: document_created_at_millis,
            withdrawals_contract::property_names::UPDATED_AT: document_created_at_millis,
        });

        let withdrawal_document = Document {
            id: document_id,
            owner_id: state_transition.identity_id,
            properties: document_data.into_btree_string_map().unwrap(),
            revision: None,
            created_at: None,
            updated_at: None,
        };

        Ok(IdentityCreditWithdrawalTransitionAction {
            version: IdentityCreditWithdrawalTransitionAction::current_version(),
            identity_id: state_transition.identity_id,
            prepared_withdrawal_document: withdrawal_document,
        }
        .into())
    }
}
