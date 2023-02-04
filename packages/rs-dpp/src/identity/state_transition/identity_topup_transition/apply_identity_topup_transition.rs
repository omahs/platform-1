use std::sync::Arc;

use anyhow::{anyhow, Result};

use crate::identity::convert_satoshi_to_credits;
use crate::identity::state_transition::asset_lock_proof::AssetLockTransactionOutputFetcher;
use crate::identity::state_transition::identity_topup_transition::IdentityTopUpTransition;
use crate::state_repository::StateRepositoryLike;
use crate::state_transition::StateTransitionLike;

pub struct ApplyIdentityTopUpTransition<SR>
where
    SR: StateRepositoryLike,
{
    state_repository: Arc<SR>,
    asset_lock_transaction_output_fetcher: Arc<AssetLockTransactionOutputFetcher<SR>>,
}

impl<SR> ApplyIdentityTopUpTransition<SR>
where
    SR: StateRepositoryLike,
{
    pub fn new(
        state_repository: Arc<SR>,
        asset_lock_transaction_output_fetcher: Arc<AssetLockTransactionOutputFetcher<SR>>,
    ) -> Self {
        Self {
            state_repository,
            asset_lock_transaction_output_fetcher,
        }
    }

    pub async fn apply(&self, state_transition: &IdentityTopUpTransition) -> Result<()> {
        let output = self
            .asset_lock_transaction_output_fetcher
            .fetch(
                state_transition.get_asset_lock_proof(),
                state_transition.get_execution_context(),
            )
            .await?;

        let credits_amount = convert_satoshi_to_credits(output.value);

        let out_point = state_transition
            .get_asset_lock_proof()
            .out_point()
            .ok_or_else(|| anyhow!("Out point is missing from asset lock proof"))?;

        let identity_id = state_transition.get_identity_id();

        self.state_repository
            .add_to_identity_balance(
                identity_id,
                credits_amount,
                state_transition.get_execution_context(),
            )
            .await?;

        // TODO: we should handle debt!!!
        self.state_repository
            .add_to_system_credits(credits_amount, state_transition.get_execution_context())
            .await?;

        self.state_repository
            .mark_asset_lock_transaction_out_point_as_used(&out_point)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate::{always, eq};
    use std::sync::Arc;

    use crate::{
        identity::state_transition::{
            asset_lock_proof::AssetLockTransactionOutputFetcher,
            identity_topup_transition::IdentityTopUpTransition,
        },
        state_repository::MockStateRepositoryLike,
        state_transition::StateTransitionLike,
        tests::fixtures::identity_topup_transition_fixture_json,
    };

    use super::ApplyIdentityTopUpTransition;

    #[tokio::test]
    async fn should_topup_amount_to_identity_balance() {
        let raw_transition = identity_topup_transition_fixture_json(None);
        let state_transition = IdentityTopUpTransition::new(raw_transition).unwrap();

        let IdentityTopUpTransition { identity_id, .. } = state_transition.clone();

        // TODO: We need to mock what fetcher actually returning and assert arguments

        let mut state_repository_for_apply = MockStateRepositoryLike::new();
        let state_repository_for_fetcher = MockStateRepositoryLike::new();

        let asset_lock_transaction_fetcher =
            AssetLockTransactionOutputFetcher::new(Arc::new(state_repository_for_fetcher));

        state_repository_for_apply
            .expect_add_to_identity_balance()
            .times(1)
            .with(eq(identity_id), eq(90000000), always())
            .returning(|_, _, _| Ok(()));

        state_repository_for_apply
            .expect_add_to_system_credits()
            .times(1)
            .with(eq(90000000), always())
            .returning(|_, _| Ok(()));

        state_repository_for_apply
            .expect_mark_asset_lock_transaction_out_point_as_used()
            .returning(|_| Ok(()));

        let apply_identity_topup_transition = ApplyIdentityTopUpTransition::new(
            Arc::new(state_repository_for_apply),
            Arc::new(asset_lock_transaction_fetcher),
        );

        let result = apply_identity_topup_transition
            .apply(&state_transition)
            .await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn should_add_topup_amount_to_identity_balance_on_dry_run() {
        let raw_transition = identity_topup_transition_fixture_json(None);
        let state_transition = IdentityTopUpTransition::new(raw_transition).unwrap();

        let IdentityTopUpTransition { identity_id, .. } = state_transition.clone();

        let mut state_repository_for_apply = MockStateRepositoryLike::new();
        let state_repository_for_fetcher = MockStateRepositoryLike::new();

        let asset_lock_transaction_fetcher =
            AssetLockTransactionOutputFetcher::new(Arc::new(state_repository_for_fetcher));

        // TODO: We need to mock what fetcher actually returning and assert arguments

        state_repository_for_apply
            .expect_add_to_identity_balance()
            .times(1)
            .with(eq(identity_id), eq(90000000), always())
            .returning(|_, _, _| Ok(()));

        state_repository_for_apply
            .expect_add_to_system_credits()
            .times(1)
            .with(eq(90000000), always())
            .returning(|_, _| Ok(()));

        state_repository_for_apply
            .expect_mark_asset_lock_transaction_out_point_as_used()
            .returning(|_| Ok(()));

        state_transition.get_execution_context().enable_dry_run();

        let apply_identity_topup_transition = ApplyIdentityTopUpTransition::new(
            Arc::new(state_repository_for_apply),
            Arc::new(asset_lock_transaction_fetcher),
        );

        let result = apply_identity_topup_transition
            .apply(&state_transition)
            .await;

        assert!(result.is_ok())
    }
}
