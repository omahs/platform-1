use dpp::identity::PartialIdentity;
use dpp::prelude::ConsensusValidationResult;
use dpp::{
    consensus::basic::{data_contract::InvalidDataContractIdError, BasicError},
    data_contract::{
        state_transition::data_contract_create_transition::DataContractCreateTransitionAction,
    },
    validation::SimpleConsensusValidationResult,
};
use dpp::{
    data_contract::{
        generate_data_contract_id,
        state_transition::data_contract_create_transition::{
            DataContractCreateTransition,
        },
    },
};
use dpp::consensus::state::data_contract::data_contract_already_present_error::DataContractAlreadyPresentError;
use dpp::consensus::state::state_error::StateError;
use dpp::state_transition::StateTransitionAction;
use dpp::data_contract::state_transition::data_contract_create_transition::validation::state::validate_data_contract_create_transition_basic::DATA_CONTRACT_CREATE_SCHEMA_VALIDATOR;
use dpp::version::{LATEST_PLATFORM_VERSION, LATEST_VERSION, PlatformVersion};
use drive::grovedb::TransactionArg;
use drive::drive::Drive;

use crate::error::Error;
use crate::platform::PlatformRef;
use crate::rpc::core::CoreRPCLike;
use crate::validation::state_transition::key_validation::validate_state_transition_identity_signature;
use crate::validation::state_transition::StateTransitionValidation;

use super::common::validate_schema;

impl StateTransitionValidation for DataContractCreateTransition {
    fn validate_structure(
        &self,
        _drive: &Drive,
        _tx: TransactionArg,
        active_protocol_version: u32,
    ) -> Result<SimpleConsensusValidationResult, Error> {
        let result = PlatformVersion::get(active_protocol_version)?
            .validate_contract_create_state_transition_version(self.state_transition_version());
        if !result.is_valid() {
            return Ok(result);
        }

        let result = validate_schema(&DATA_CONTRACT_CREATE_SCHEMA_VALIDATOR, self);
        if !result.is_valid() {
            return Ok(result);
        }

        // Validate data contract id
        let generated_id =
            generate_data_contract_id(self.data_contract().owner_id, self.data_contract().entropy);
        if generated_id.as_slice() != self.data_contract.id.as_ref() {
            return Ok(SimpleConsensusValidationResult::new_with_error(
                BasicError::InvalidDataContractIdError(InvalidDataContractIdError::new(
                    generated_id.to_vec(),
                    self.data_contract.id.as_ref().to_owned(),
                ))
                .into(),
            ));
        }

        self.data_contract
            .validate_structure()
            .map_err(Error::Protocol)
    }

    fn validate_identity_and_signatures(
        &self,
        drive: &Drive,
        transaction: TransactionArg,
    ) -> Result<ConsensusValidationResult<Option<PartialIdentity>>, Error> {
        Ok(
            validate_state_transition_identity_signature(drive, self, false, transaction)?
                .map(Some),
        )
    }

    fn validate_state<'a, C: CoreRPCLike>(
        &self,
        platform: &'a PlatformRef<C>,
        tx: TransactionArg,
    ) -> Result<ConsensusValidationResult<StateTransitionAction>, Error> {
        let drive = platform.drive;
        // Data contract shouldn't exist
        if drive
            .get_contract_with_fetch_info(self.data_contract.id.to_buffer(), None, false, tx)?
            .1
            .is_some()
        {
            Ok(ConsensusValidationResult::new_with_errors(vec![
                StateError::DataContractAlreadyPresentError(DataContractAlreadyPresentError::new(
                    self.data_contract.id.to_owned(),
                ))
                .into(),
            ]))
        } else {
            let action: StateTransitionAction =
                Into::<DataContractCreateTransitionAction>::into(self).into();
            Ok(action.into())
        }
    }
}
