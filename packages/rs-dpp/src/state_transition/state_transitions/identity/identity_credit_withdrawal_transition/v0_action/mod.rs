use crate::contracts::withdrawals_contract;
use crate::document::{generate_document_id, Document, DocumentV0};
use crate::identifier::Identifier;
use crate::prelude::Revision;
use crate::state_transition::identity_credit_withdrawal_transition::v0::IdentityCreditWithdrawalTransitionV0;
use platform_value::platform_value;
use serde::{Deserialize, Serialize};

pub const IDENTITY_CREDIT_WITHDRAWAL_TRANSITION_VERSION: u32 = 0;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityCreditWithdrawalTransitionActionV0 {
    pub identity_id: Identifier,
    pub revision: Revision,
    pub prepared_withdrawal_document: Document,
}

impl IdentityCreditWithdrawalTransitionActionV0 {
    pub fn current_version() -> u32 {
        IDENTITY_CREDIT_WITHDRAWAL_TRANSITION_VERSION
    }

    pub fn from_identity_credit_withdrawal(
        identity_credit_withdrawal: &IdentityCreditWithdrawalTransitionV0,
        creation_time_ms: u64,
    ) -> Self {
        let document_id = generate_document_id::generate_document_id(
            &withdrawals_contract::CONTRACT_ID,
            &identity_credit_withdrawal.identity_id,
            withdrawals_contract::document_types::WITHDRAWAL,
            identity_credit_withdrawal.output_script.as_bytes(),
        );

        let document_data = platform_value!({
            withdrawals_contract::property_names::AMOUNT: identity_credit_withdrawal.amount,
            withdrawals_contract::property_names::CORE_FEE_PER_BYTE: identity_credit_withdrawal.core_fee_per_byte,
            withdrawals_contract::property_names::POOLING: Pooling::Never,
            withdrawals_contract::property_names::OUTPUT_SCRIPT: identity_credit_withdrawal.output_script.as_bytes(),
            withdrawals_contract::property_names::STATUS: withdrawals_contract::WithdrawalStatus::QUEUED,
        });

        let withdrawal_document = DocumentV0 {
            id: document_id,
            owner_id: identity_credit_withdrawal.identity_id,
            properties: document_data.into_btree_string_map().unwrap(),
            revision: Some(1),
            created_at: Some(creation_time_ms),
            updated_at: Some(creation_time_ms),
        }
        .into();

        IdentityCreditWithdrawalTransitionActionV0 {
            version: IdentityCreditWithdrawalTransitionActionV0::current_version(),
            identity_id: identity_credit_withdrawal.identity_id,
            revision: identity_credit_withdrawal.revision,
            prepared_withdrawal_document: withdrawal_document,
        }
    }
}
