mod v0;

use crate::error::execution::ExecutionError;
use crate::error::Error;
use crate::platform_types::platform::Platform;
use crate::platform_types::platform_state::v0::PlatformStateV0Methods;
use crate::platform_types::platform_state::PlatformState;

use crate::platform_types::validator_set::v0::{ValidatorSetV0, ValidatorSetV0Getters};
use crate::platform_types::validator_set::ValidatorSet;
use crate::rpc::core::CoreRPCLike;
use dpp::version::PlatformVersion;
use std::cmp::Ordering;

impl<C> Platform<C>
where
    C: CoreRPCLike,
{
    /// Updates the quorum information for the platform state based on the given core block height.
    ///
    /// # Arguments
    ///
    /// * `state` - A mutable reference to the platform state.
    /// * `core_block_height` - The core block height for which to update the quorum information.
    ///
    /// # Returns
    ///
    /// * `Result<SimpleConsensusValidationResult, ExecutionError>` - A `SimpleConsensusValidationResult`
    ///   on success, or an `Error` on failure.
    pub(in crate::execution::platform_events::core_based_updates) fn update_quorum_info(
        &self,
        block_platform_state: &mut PlatformState,
        core_block_height: u32,
        start_from_scratch: bool,
        platform_version: &PlatformVersion,
    ) -> Result<(), Error> {
        match platform_version
            .drive_abci
            .methods
            .core_based_updates
            .update_quorum_info
        {
            0 => self.update_quorum_info_v0(
                block_platform_state,
                core_block_height,
                start_from_scratch,
                platform_version,
            ),
            version => Error::Execution(ExecutionError::UnknownVersionMismatch {
                method: "update_quorum_info".to_string(),
                known_versions: vec![0],
                received: version,
            }),
        }
    }
}