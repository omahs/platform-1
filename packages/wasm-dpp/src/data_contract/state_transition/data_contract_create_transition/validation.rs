use std::sync::Arc;

use dpp::{
    data_contract::state_transition::data_contract_create_transition::validation::state::{
        validate_data_contract_create_transition_basic::DataContractCreateTransitionBasicValidator,
        validate_data_contract_create_transition_state::validate_data_contract_create_transition_state as dpp_validate_data_contract_create_transition_state,
    },
    state_transition::state_transition_execution_context::StateTransitionExecutionContext,
    validation::DataValidatorWithContext,
    version::ProtocolVersionValidator,
};
use wasm_bindgen::prelude::*;

use crate::validation::ValidationResultWasm;
use crate::{
    errors::from_dpp_err,
    state_repository::{ExternalStateRepositoryLike, ExternalStateRepositoryLikeWrapper},
    DataContractCreateTransitionWasm,
};

use super::DataContractCreateTransitionParameters;

#[wasm_bindgen(js_name=validateDataContractCreateTransitionState)]
pub async fn validate_data_contract_create_transition_state(
    state_repository: ExternalStateRepositoryLike,
    state_transition: DataContractCreateTransitionWasm,
) -> Result<ValidationResultWasm, JsValue> {
    let wrapped_state_repository = ExternalStateRepositoryLikeWrapper::new(state_repository);
    let validation_result = dpp_validate_data_contract_create_transition_state(
        &wrapped_state_repository,
        &state_transition.into(),
    )
    .await
    .map_err(from_dpp_err)?;
    Ok(validation_result.map(|_| JsValue::undefined()).into())
}

#[wasm_bindgen(js_name=validateDataContractCreateTransitionBasic)]
pub async fn validate_data_contract_create_transition_basic(
    raw_parameters: JsValue,
) -> Result<ValidationResultWasm, JsError> {
    let parameters: DataContractCreateTransitionParameters =
        serde_wasm_bindgen::from_value(raw_parameters)?;

    let validator = DataContractCreateTransitionBasicValidator::new(Arc::new(
        ProtocolVersionValidator::default(),
    ))?;

    let validation_result = validator.validate(
        &serde_json::to_value(&parameters)?,
        &StateTransitionExecutionContext::default(),
    )?;

    Ok(validation_result.map(|_| JsValue::undefined()).into())
}
