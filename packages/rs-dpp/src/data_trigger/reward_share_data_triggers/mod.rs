use std::convert::TryInto;

use anyhow::{anyhow, bail};
use platform_value::btreemap_extensions::BTreeValueMapHelper;

use platform_value::platform_value;

use crate::document::Document;
use crate::{
    data_trigger::create_error, document::document_transition::DocumentTransition,
    get_from_transition, prelude::Identifier, state_repository::StateRepositoryLike, ProtocolError,
};

use super::{DataTriggerExecutionContext, DataTriggerExecutionResult};

const MAX_PERCENTAGE: u64 = 10000;
const PROPERTY_PAY_TO_ID: &str = "payToId";
const PROPERTY_PERCENTAGE: &str = "percentage";
const MAX_DOCUMENTS: usize = 16;

pub async fn create_masternode_reward_shares_data_trigger<'a, SR>(
    document_transition: &DocumentTransition,
    context: &DataTriggerExecutionContext<'a, SR>,
    _top_level_identifier: Option<&Identifier>,
) -> Result<DataTriggerExecutionResult, anyhow::Error>
where
    SR: StateRepositoryLike,
{
    let mut result = DataTriggerExecutionResult::default();
    let is_dry_run = context.state_transition_execution_context.is_dry_run();
    let owner_id = context.owner_id.to_buffer();

    let (transition_data, transition_base) = match document_transition {
        DocumentTransition::Create(document_create_transition) => (
            document_create_transition.data.as_ref(),
            &document_create_transition.base,
        ),
        DocumentTransition::Replace(document_replace_transition) => (
            document_replace_transition.data.as_ref(),
            &document_replace_transition.base,
        ),
        _ => bail!(
            "the Document Transition {} isn't 'CREATE or REPLACE'",
            get_from_transition!(document_transition, id)
        ),
    };
    let properties = transition_data.ok_or_else(|| {
        anyhow!(
            "data isn't defined in Data Transition '{}'",
            transition_base.id
        )
    })?;

    let pay_to_id = properties.get_hash256_bytes(PROPERTY_PAY_TO_ID)?;
    let percentage = properties.get_integer(PROPERTY_PERCENTAGE)?;

    if !is_dry_run {
        let is_valid_master_node = context
            .state_repository
            .is_in_the_valid_master_nodes_list(context.owner_id.to_buffer())
            .await?;

        // TODO: bring it back once the SML store is implemented
        // // Do not allow creating document if ownerId is not in SML
        // let sml_store: SMLStore = context.state_repository.fetch_sml_store().await?;
        //
        // let valid_master_nodes_list = sml_store.get_current_sml()?.get_valid_master_nodes();
        //
        // let owner_id_in_sml = valid_master_nodes_list.iter().any(|entry| {
        //     hex::decode(&entry.pro_reg_tx_hash).expect("invalid hex value")
        //         == context.owner_id.to_buffer()
        // });

        if !is_valid_master_node {
            let err = create_error(
                context,
                transition_base.id,
                "Only masternode identities can share rewards".to_string(),
            );
            result.add_error(err.into());
        }
    }

    // payToId identity exists
    let pay_to_identifier = Identifier::from(pay_to_id);
    let maybe_identity = context
        .state_repository
        .fetch_identity(
            &pay_to_identifier,
            Some(context.state_transition_execution_context),
        )
        .await?;

    if !is_dry_run && maybe_identity.is_none() {
        let err = create_error(
            context,
            transition_base.id,
            format!("Identity '{}' doesn't exist", pay_to_identifier),
        );
        result.add_error(err.into());
        return Ok(result);
    }

    let documents_data = context
        .state_repository
        .fetch_documents(
            &context.data_contract.id,
            &transition_base.document_type_name,
            platform_value!({
                "where" : [ [ "$ownerId", "==", owner_id ]]
            }),
            Some(context.state_transition_execution_context),
        )
        .await?;
    let documents: Vec<Document> = documents_data
        .into_iter()
        .map(|d| d.try_into().map_err(Into::<ProtocolError>::into))
        .collect::<Result<Vec<Document>, ProtocolError>>()?;

    if is_dry_run {
        return Ok(result);
    }

    if documents.len() >= MAX_DOCUMENTS {
        let err = create_error(
            context,
            transition_base.id,
            format!(
                "Reward shares cannot contain more than {} identities",
                MAX_DOCUMENTS
            ),
        );
        result.add_error(err.into());
        return Ok(result);
    }

    let mut total_percent: u64 = percentage;
    for d in documents.iter() {
        total_percent += d.properties.get_integer::<u64>(PROPERTY_PERCENTAGE)?;
    }

    if total_percent > MAX_PERCENTAGE {
        let err = create_error(
            context,
            transition_base.id,
            format!("Percentage can not be more than {}", MAX_PERCENTAGE),
        );
        result.add_error(err.into());
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    use platform_value::Value;

    use crate::consensus::state::data_trigger::data_trigger_error::DataTriggerError;
    use crate::consensus::state::state_error::StateError;
    use crate::document::{Document, ExtendedDocument};
    use crate::identity::Identity;
    use crate::{
        data_contract::DataContract,
        data_trigger::DataTriggerExecutionContext,
        document::document_transition::{Action, DocumentTransition, DocumentTransitionExt},
        mocks::{SMLEntry, SMLStore, SimplifiedMNList},
        prelude::Identifier,
        state_repository::MockStateRepositoryLike,
        state_transition::state_transition_execution_context::StateTransitionExecutionContext,
        tests::{
            fixtures::{
                get_document_transitions_fixture, get_masternode_reward_shares_documents_fixture,
            },
            utils::generate_random_identifier_struct,
        },
    };

    struct TestData {
        top_level_identifier: Identifier,
        data_contract: DataContract,
        sml_store: SMLStore,
        extended_documents: Vec<ExtendedDocument>,
        document_transition: DocumentTransition,
        identity: Identity,
    }

    fn setup_test() -> TestData {
        let top_level_identifier_hex =
            "c286807d463b06c7aba3b9a60acf64c1fc03da8c1422005cd9b4293f08cf0562";
        let top_level_identifier =
            Identifier::from_bytes(&hex::decode(top_level_identifier_hex).unwrap()).unwrap();

        let sml_entries: Vec<SMLEntry> = vec![
            SMLEntry {
          pro_reg_tx_hash: top_level_identifier_hex.to_string(),
          confirmed_hash: "4eb56228c535db3b234907113fd41d57bcc7cdcb8e0e00e57590af27ee88c119".to_string(),
          service: "192.168.65.2:20101".to_string(),
          pub_key_operator: "809519c5f6f3be1c08782ac42ae9a83b6c7205eba43f9a96a4f032ec7a73f1a7c25fa78cce0d6d9c135f7e2c28527179".to_string(),
          voting_address: "yXmprXYP51uzfMyndtWwxz96MnkCKkFc9x".to_string(),
          is_valid: true,
        },

        SMLEntry {
          pro_reg_tx_hash: "a3e1edc6bd352eeaf0ae58e30781ef4b127854241a3fe7fddf36d5b7e1dc2b3f".to_string(),
          confirmed_hash: "27a0b637b56af038c45e2fd1f06c2401c8dadfa28ca5e0d19ca836cc984a8378".to_string(),
          service: "192.168.65.2:20201".to_string(),
          pub_key_operator: "987a4873caba62cd45a2f7d4aa6d94519ee6753e9bef777c927cb94ade768a542b0ff34a93231d3a92b4e75ffdaa366e".to_string(),
          voting_address: "ycL7L4mhYoaZdm9TH85svvpfeKtdfo249u".to_string(),
          is_valid: true,
         }
        ];

        let (documents, data_contract) = get_masternode_reward_shares_documents_fixture();
        let sml_store = SMLStore {
            sml_list_by_height: SimplifiedMNList {
                masternodes: sml_entries.clone(),
            },
            sml_list_current: SimplifiedMNList {
                masternodes: sml_entries,
            },
        };
        let document_transitions =
            get_document_transitions_fixture([(Action::Create, vec![documents[0].clone()])]);
        TestData {
            extended_documents: documents,
            data_contract,
            top_level_identifier,
            sml_store,
            document_transition: document_transitions[0].clone(),
            identity: Identity::default(),
        }
    }

    fn get_data_trigger_error(
        result: &Result<DataTriggerExecutionResult, anyhow::Error>,
        error_number: usize,
    ) -> &DataTriggerError {
        let execution_result = result.as_ref().expect("it should return execution result");
        let error = execution_result
            .get_errors()
            .get(error_number)
            .expect("errors should exist");
        match error {
            StateError::DataTriggerError(error) => error,
            _ => {
                panic!("the returned error is not a data trigger error")
            }
        }
    }

    #[tokio::test]
    async fn should_return_an_error_if_percentage_greater_than_1000() {
        let TestData {
            mut document_transition,
            extended_documents,

            data_contract,
            top_level_identifier,
            identity,
            ..
        } = setup_test();

        let documents: Vec<Document> = extended_documents
            .clone()
            .into_iter()
            .map(|dt| dt.document)
            .collect();

        let mut state_repository_mock = MockStateRepositoryLike::new();
        state_repository_mock
            .expect_is_in_the_valid_master_nodes_list()
            .returning(move |_| Ok(true));
        state_repository_mock
            .expect_fetch_identity()
            .returning(move |_, _| Ok(Some(identity.clone())));
        state_repository_mock
            .expect_fetch_documents()
            .returning(move |_, _, _, _| Ok(documents.clone()));

        // documentsFixture contains percentage = 500
        document_transition.insert_dynamic_property(String::from("percentage"), Value::U64(9501));

        let execution_context = StateTransitionExecutionContext::default();
        let context = DataTriggerExecutionContext {
            data_contract: &data_contract,
            owner_id: &top_level_identifier,
            state_repository: &state_repository_mock,
            state_transition_execution_context: &execution_context,
        };

        let result =
            create_masternode_reward_shares_data_trigger(&document_transition, &context, None)
                .await;

        let percentage_error = get_data_trigger_error(&result, 0);
        assert_eq!(
            "Percentage can not be more than 10000",
            percentage_error.to_string()
        );
    }

    #[tokio::test]
    async fn should_return_an_error_if_pay_to_id_does_not_exists() {
        let TestData {
            document_transition,

            data_contract,
            top_level_identifier,
            ..
        } = setup_test();

        let mut state_repository_mock = MockStateRepositoryLike::new();
        state_repository_mock
            .expect_is_in_the_valid_master_nodes_list()
            .returning(move |_| Ok(true));
        state_repository_mock
            .expect_fetch_identity()
            .returning(move |_, _| Ok(None));
        state_repository_mock
            .expect_fetch_documents()
            .returning(move |_, _, _, _| Ok(vec![]));

        let execution_context = StateTransitionExecutionContext::default();
        let context = DataTriggerExecutionContext {
            data_contract: &data_contract,
            owner_id: &top_level_identifier,
            state_repository: &state_repository_mock,
            state_transition_execution_context: &execution_context,
        };
        let result =
            create_masternode_reward_shares_data_trigger(&document_transition, &context, None)
                .await;

        let error = get_data_trigger_error(&result, 0);
        let pay_to_id_bytes = document_transition
            .get_dynamic_property(PROPERTY_PAY_TO_ID)
            .expect("payToId should exist")
            .to_hash256()
            .expect("expected to be able to get a hash");
        let pay_to_id = Identifier::from(pay_to_id_bytes);

        assert_eq!(
            format!("Identity '{}' doesn't exist", pay_to_id),
            error.to_string()
        );
    }

    #[tokio::test]
    async fn should_return_an_error_if_owner_id_is_not_a_masternode_identity() {
        let TestData {
            document_transition,

            data_contract,
            ..
        } = setup_test();

        let mut state_repository_mock = MockStateRepositoryLike::new();
        state_repository_mock
            .expect_is_in_the_valid_master_nodes_list()
            .returning(move |_| Ok(false));
        state_repository_mock
            .expect_fetch_identity()
            .returning(move |_, _| Ok(None));
        state_repository_mock
            .expect_fetch_documents()
            .returning(move |_, _, _, _| Ok(vec![]));

        let execution_context = StateTransitionExecutionContext::default();
        let context = DataTriggerExecutionContext {
            data_contract: &data_contract,
            owner_id: &generate_random_identifier_struct(),
            state_repository: &state_repository_mock,
            state_transition_execution_context: &execution_context,
        };
        let result =
            create_masternode_reward_shares_data_trigger(&document_transition, &context, None)
                .await;
        let error = get_data_trigger_error(&result, 0);

        assert_eq!(
            "Only masternode identities can share rewards",
            error.to_string()
        );
    }

    #[tokio::test]
    async fn should_pass() {
        let TestData {
            document_transition,

            data_contract,
            top_level_identifier,
            identity,
            ..
        } = setup_test();

        let mut state_repository_mock = MockStateRepositoryLike::new();
        state_repository_mock
            .expect_is_in_the_valid_master_nodes_list()
            .returning(move |_| Ok(true));
        state_repository_mock
            .expect_fetch_identity()
            .returning(move |_, _| Ok(Some(identity.clone())));
        state_repository_mock
            .expect_fetch_documents()
            .returning(move |_, _, _, _| Ok(vec![]));

        let execution_context = StateTransitionExecutionContext::default();
        let context = DataTriggerExecutionContext {
            data_contract: &data_contract,
            owner_id: &top_level_identifier,
            state_repository: &state_repository_mock,
            state_transition_execution_context: &execution_context,
        };
        let result =
            create_masternode_reward_shares_data_trigger(&document_transition, &context, None)
                .await
                .expect("the execution result should be returned");
        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn should_return_error_if_there_are_16_stored_shares() {
        let TestData {
            document_transition,

            data_contract,
            top_level_identifier,
            identity,
            ..
        } = setup_test();

        let mut state_repository_mock = MockStateRepositoryLike::new();
        state_repository_mock
            .expect_is_in_the_valid_master_nodes_list()
            .returning(move |_| Ok(true));
        state_repository_mock
            .expect_fetch_identity()
            .returning(move |_, _| Ok(Some(identity.clone())));
        let documents_to_return: Vec<Document> = (0..16).map(|_| Document::default()).collect();
        state_repository_mock
            .expect_fetch_documents()
            .return_once(move |_, _, _, _| Ok(documents_to_return));

        let execution_context = StateTransitionExecutionContext::default();
        let context = DataTriggerExecutionContext {
            data_contract: &data_contract,
            owner_id: &top_level_identifier,
            state_repository: &state_repository_mock,
            state_transition_execution_context: &execution_context,
        };

        let result =
            create_masternode_reward_shares_data_trigger(&document_transition, &context, None)
                .await;
        let error = get_data_trigger_error(&result, 0);

        assert_eq!(
            "Reward shares cannot contain more than 16 identities",
            error.to_string()
        );
    }

    #[tokio::test]
    async fn should_pass_on_dry_run() {
        let TestData {
            document_transition,
            data_contract,
            top_level_identifier,
            ..
        } = setup_test();

        let mut state_repository_mock = MockStateRepositoryLike::new();
        state_repository_mock
            .expect_fetch_identity()
            .returning(move |_, _| Ok(None));
        state_repository_mock
            .expect_fetch_documents()
            .returning(move |_, _, _, _| Ok(vec![]));

        let execution_context = StateTransitionExecutionContext::default();
        execution_context.enable_dry_run();

        let context = DataTriggerExecutionContext {
            data_contract: &data_contract,
            owner_id: &top_level_identifier,
            state_repository: &state_repository_mock,
            state_transition_execution_context: &execution_context,
        };
        let result =
            create_masternode_reward_shares_data_trigger(&document_transition, &context, None)
                .await
                .expect("the execution result should be returned");
        assert!(result.is_ok());
    }
}
