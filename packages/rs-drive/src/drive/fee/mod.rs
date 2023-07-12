use enum_map::EnumMap;
use dpp::block::epoch::Epoch;
use dpp::fee::fee_result::FeeResult;
use dpp::ProtocolError;
use crate::error::Error;
use crate::error::fee::FeeError;

/// Calculates fees for the given operations. Returns the storage and processing costs.
pub fn calculate_fee(
    base_operations: Option<EnumMap<BaseOp, u64>>,
    drive_operations: Option<Vec<LowLevelDriveOperation>>,
    epoch: &Epoch,
) -> Result<FeeResult, Error> {
    let mut aggregate_fee_result = FeeResult::default();
    if let Some(base_operations) = base_operations {
        for (base_op, count) in base_operations.iter() {
            match base_op.cost().checked_mul(*count) {
                None => return Err(Error::Fee(FeeError::Overflow("overflow error"))),
                Some(cost) => match aggregate_fee_result.processing_fee.checked_add(cost) {
                    None => return Err(Error::Fee(FeeError::Overflow("overflow error"))),
                    Some(value) => aggregate_fee_result.processing_fee = value,
                },
            }
        }
    }

    if let Some(drive_operations) = drive_operations {
        // println!("{:#?}", drive_operations);
        for drive_fee_result in LowLevelDriveOperation::consume_to_fees(drive_operations, epoch)? {
            aggregate_fee_result.checked_add_assign(drive_fee_result)?;
        }
    }

    Ok(aggregate_fee_result)
}


pub fn get_overflow_error(str: &'static str) -> Error {
    Error::Fee(FeeError::Overflow(str))
}