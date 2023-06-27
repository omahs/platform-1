use grovedb::Transaction;
use crate::drive::Drive;
use crate::error::Error;

impl Drive {
    /// Rolls back a transaction.
    pub(super) fn rollback_transaction_v0(&self, transaction: &Transaction) -> Result<(), Error> {
        self.grove
            .rollback_transaction(transaction)
            .map_err(Error::GroveDB)
    }
}
