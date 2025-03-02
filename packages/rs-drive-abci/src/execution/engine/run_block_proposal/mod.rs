use crate::error::Error;
use crate::platform_types::platform::Platform;
use crate::platform_types::{block_execution_outcome, block_proposal};
use crate::rpc::core::CoreRPCLike;
use dpp::validation::ValidationResult;
use drive::grovedb::Transaction;

mod v0;

impl<C> Platform<C>
where
    C: CoreRPCLike,
{
    /// Runs a block proposal, either from process proposal or prepare proposal.
    ///
    /// This function takes a `BlockProposal` and a `Transaction` as input and processes the block
    /// proposal. It first validates the block proposal and then processes raw state transitions,
    /// withdrawal transactions, and block fees. It also updates the validator set.
    ///
    /// # Arguments
    ///
    /// * `block_proposal` - The block proposal to be processed.
    /// * `transaction` - The transaction associated with the block proposal.
    ///
    /// # Returns
    ///
    /// * `Result<ValidationResult<BlockExecutionOutcome, Error>, Error>` - If the block proposal is
    ///   successfully processed, it returns a `ValidationResult` containing the `BlockExecutionOutcome`.
    ///   If the block proposal processing fails, it returns an `Error`. Consensus errors are returned
    ///   in the `ValidationResult`, while critical system errors are returned in the `Result`.
    ///
    /// # Errors
    ///
    /// This function may return an `Error` variant if there is a problem with processing the block
    /// proposal, updating the core info, processing raw state transitions, or processing block fees.
    ///
    pub fn run_block_proposal(
        &self,
        block_proposal: block_proposal::v0::BlockProposal,
        transaction: &Transaction,
    ) -> Result<ValidationResult<block_execution_outcome::v0::BlockExecutionOutcome, Error>, Error>
    {
        //todo: use protocol version to decide
        self.run_block_proposal_v0(block_proposal, transaction)
    }
}
