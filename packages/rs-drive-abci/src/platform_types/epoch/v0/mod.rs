// MIT LICENSE
//
// Copyright (c) 2021 Dash Core Group
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

//! Epoch Info.
//!
//! This module defines and implements the `EpochInfo` struct containing
//! information about the current epoch.
//!

use crate::error::execution::ExecutionError;
use crate::error::Error;
use crate::execution::types::block_state_info::v0::BlockStateInfo;
use dpp::block::epoch::Epoch;
use dpp::ProtocolError;
use drive::fee::epoch::GENESIS_EPOCH_INDEX;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Lifetime of an epoch in milliseconds.
pub const EPOCH_CHANGE_TIME_MS_V0: u64 = 1576800000;

/// Info pertinent to the current epoch.
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EpochInfo {
    /// Current epoch index
    pub current_epoch_index: u16,

    /// Previous epoch index
    /// Available only on epoch change
    pub previous_epoch_index: Option<u16>,

    /// Boolean true if it's the first block of a new epoch
    pub is_epoch_change: bool,
}

impl EpochInfo {
    /// Returns true if it's an epoch change but not the Epoch 0 on genesis
    pub fn is_epoch_change_but_not_genesis(&self) -> bool {
        self.is_epoch_change && self.current_epoch_index != GENESIS_EPOCH_INDEX
    }

    /// Converts some values to decimal types and calculates some relevant epoch info values.
    pub fn calculate(
        genesis_time_ms: u64,
        block_time_ms: u64,
        previous_block_time_ms: Option<u64>,
    ) -> Result<Self, Error> {
        let previous_block_time = match previous_block_time_ms {
            Some(block_time) => block_time,
            None => return Ok(EpochInfo::default()),
        };

        let epoch_change_time = Decimal::from(EPOCH_CHANGE_TIME_MS_V0);
        let block_time = Decimal::from(block_time_ms);
        let genesis_time = Decimal::from(genesis_time_ms);
        let previous_block_time = Decimal::from(previous_block_time);

        let previous_epoch_index = (previous_block_time - genesis_time) / epoch_change_time;
        let previous_epoch_index_floored = previous_epoch_index.floor();

        let epoch_index = (block_time - genesis_time) / epoch_change_time;
        let epoch_index_floored = epoch_index.floor();

        let is_epoch_change = epoch_index_floored > previous_epoch_index_floored;

        let current_epoch_index: u16 = epoch_index_floored.try_into().map_err(|_| {
            Error::Execution(ExecutionError::Conversion(
                "can't convert epochs index from Decimal to u16",
            ))
        })?;

        let previous_epoch_index: Option<u16> = if epoch_index_floored
            != previous_epoch_index_floored
        {
            let previous_epoch_index = previous_epoch_index_floored.try_into().map_err(|_| {
                Error::Execution(ExecutionError::Conversion(
                    "can't convert epochs index from Decimal to u16",
                ))
            })?;

            Some(previous_epoch_index)
        } else {
            None
        };

        Ok(Self {
            current_epoch_index,
            previous_epoch_index,
            is_epoch_change,
        })
    }

    /// Takes genesis time and block info and sets current and previous epoch indexes as well as
    /// the is_epoch_change bool by calling calculate().
    pub fn from_genesis_time_and_block_info(
        genesis_time_ms: u64,
        block_info: &BlockStateInfo,
    ) -> Result<Self, Error> {
        Self::calculate(
            genesis_time_ms,
            block_info.block_time_ms,
            block_info.previous_block_time_ms,
        )
    }
}

impl Default for EpochInfo {
    /// Default epoch info.
    fn default() -> EpochInfo {
        EpochInfo {
            current_epoch_index: 0,
            previous_epoch_index: None,
            is_epoch_change: true,
        }
    }
}

impl TryFrom<&EpochInfo> for Epoch {
    type Error = ProtocolError;

    fn try_from(value: &EpochInfo) -> Result<Self, Self::Error> {
        Epoch::new(value.current_epoch_index)
    }
}

#[cfg(test)]
mod test {
    mod calculate {
        use crate::platform_types::epoch::v0::EpochInfo;

        #[test]
        fn test_epoch_change_to_0_epoch() {
            let genesis_time_ms: u64 = 1655396517902;
            let block_time_ms: u64 = 1655396517922;

            let epoch_info = EpochInfo::calculate(genesis_time_ms, block_time_ms, None)
                .expect("should calculate epochs info");

            assert_eq!(epoch_info.current_epoch_index, 0);
            assert!(epoch_info.is_epoch_change);
        }

        #[test]
        fn test_no_epoch_change() {
            let genesis_time_ms: u64 = 1655396517902;
            let block_time_ms: u64 = 1655396517922;
            let prev_block_time_ms: u64 = 1655396517912;

            let epoch_info =
                EpochInfo::calculate(genesis_time_ms, block_time_ms, Some(prev_block_time_ms))
                    .expect("should calculate epochs info");

            assert_eq!(epoch_info.current_epoch_index, 0);
            assert!(!epoch_info.is_epoch_change);
        }

        #[test]
        fn test_epoch_change_to_epoch_1() {
            let genesis_time_ms: u64 = 1655396517902;
            let prev_block_time_ms: u64 = 1655396517912;
            let block_time_ms: u64 = 1657125244561;

            let epoch_info =
                EpochInfo::calculate(genesis_time_ms, block_time_ms, Some(prev_block_time_ms))
                    .expect("should calculate epochs info");

            assert_eq!(epoch_info.current_epoch_index, 1);
            assert!(epoch_info.is_epoch_change);
        }
    }
}
