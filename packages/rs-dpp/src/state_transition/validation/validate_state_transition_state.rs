use crate::state_repository::StateRepositoryLike;
use std::sync::Arc;
use crate::data_contract::state_transition::data_contract_create_transition::validation::state::validate_data_contract_create_transition_state::DataContractCreateTransitionStateValidator;
use crate::data_contract::state_transition::data_contract_update_transition::validation::state::validate_data_contract_update_transition_state::DataContractUpdateTransitionStateValidator;
use crate::document::validation::state::validate_documents_batch_transition_state::DocumentsBatchTransitionStateValidator;
use crate::identity::state_transition::identity_create_transition::validation::state::IdentityCreateTransitionStateValidator;
use crate::identity::state_transition::identity_credit_withdrawal_transition::validation::state::validate_identity_credit_withdrawal_transition_state::IdentityCreditWithdrawalTransitionValidator;
use crate::identity::state_transition::identity_topup_transition::validation::state::IdentityTopUpTransitionStateValidator;
use crate::identity::state_transition::identity_update_transition::validate_identity_update_transition_state::IdentityUpdateTransitionStateValidator;
use crate::identity::state_transition::identity_update_transition::validate_public_keys::IdentityUpdatePublicKeysValidator;
use crate::ProtocolError;
use crate::state_transition::StateTransition;
use crate::validation::{AsyncDataValidator, SimpleValidationResult};

pub struct StateTransitionStateValidator<SR>
where
    SR: StateRepositoryLike + Clone,
{
    state_repository: SR,
    data_contract_create_validator: DataContractCreateTransitionStateValidator<SR>,
    data_contract_update_validator: DataContractUpdateTransitionStateValidator<SR>,
    identity_create_validator: IdentityCreateTransitionStateValidator<SR>,
    identity_update_validator:
        IdentityUpdateTransitionStateValidator<IdentityUpdatePublicKeysValidator, SR>,
    identity_top_up_validator: IdentityTopUpTransitionStateValidator<SR>,
    identity_credit_withdrawal_validator: IdentityCreditWithdrawalTransitionValidator<SR>,
    document_batch_validator: DocumentsBatchTransitionStateValidator<SR>,
}

impl<SR> StateTransitionStateValidator<SR>
where
    SR: StateRepositoryLike + Clone,
{
    pub fn new(state_repository: SR) -> Self {
        let wrapped_state_repository = Arc::new(state_repository.clone());

        let data_contract_create_validator =
            DataContractCreateTransitionStateValidator::new(state_repository.clone());
        let data_contract_update_validator =
            DataContractUpdateTransitionStateValidator::new(state_repository.clone());
        let identity_create_validator =
            IdentityCreateTransitionStateValidator::new(state_repository.clone());
        let identity_update_validator = IdentityUpdateTransitionStateValidator::new(
            wrapped_state_repository.clone(),
            Arc::new(IdentityUpdatePublicKeysValidator {}),
        );
        let identity_top_up_validator =
            IdentityTopUpTransitionStateValidator::new(state_repository.clone());
        let identity_credit_withdrawal_validator =
            IdentityCreditWithdrawalTransitionValidator::new(wrapped_state_repository);
        let document_batch_validator =
            DocumentsBatchTransitionStateValidator::new(state_repository.clone());

        StateTransitionStateValidator {
            state_repository,
            data_contract_create_validator,
            data_contract_update_validator,
            identity_create_validator,
            identity_update_validator,
            identity_top_up_validator,
            identity_credit_withdrawal_validator,
            document_batch_validator,
        }
    }

    pub async fn validate(
        &self,
        state_transition: &StateTransition,
    ) -> Result<SimpleValidationResult, ProtocolError> {
        match state_transition {
            StateTransition::DataContractCreate(st) => {
                self.data_contract_create_validator.validate(st).await
            }
            StateTransition::DataContractUpdate(st) => {
                self.data_contract_update_validator.validate(st).await
            }
            StateTransition::IdentityCreate(st) => {
                self.identity_create_validator.validate(st).await
            }
            StateTransition::IdentityUpdate(st) => self
                .identity_update_validator
                .validate(st)
                .await
                .map_err(|e| ProtocolError::from(e)),
            StateTransition::IdentityTopUp(st) => self.identity_top_up_validator.validate(st).await,
            StateTransition::IdentityCreditWithdrawal(st) => self
                .identity_credit_withdrawal_validator
                .validate_identity_credit_withdrawal_transition_state(st)
                .await
                .map_err(|e| ProtocolError::from(e)),
            StateTransition::DocumentsBatch(st) => self.document_batch_validator.validate(st).await,
        }
    }
}