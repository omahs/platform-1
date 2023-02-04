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

//! Common functions
//!
//! This module defines general, commonly used functions in Drive.
//!

pub mod encode;
/// Helpers module
pub mod helpers;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::option::Option::None;
use std::path::Path;

use ciborium::value::Value;
use dpp::data_contract::DriveContractExt;
use grovedb::TransactionArg;

use crate::contract::Contract;
use crate::drive::Drive;

use dpp::data_contract::extra::common::json_document_to_cbor;

use crate::drive::block_info::BlockInfo;

/// Serializes to CBOR and applies to Drive a JSON contract from the file system.
pub fn setup_contract(
    drive: &Drive,
    path: &str,
    contract_id: Option<[u8; 32]>,
    transaction: TransactionArg,
) -> Contract {
    let contract_cbor = json_document_to_cbor(path, Some(crate::drive::defaults::PROTOCOL_VERSION))
        .expect("expected to get cbor contract");
    let contract = <Contract as DriveContractExt>::from_cbor(&contract_cbor, contract_id)
        .expect("contract should be deserialized");
    let contract_cbor =
        DriveContractExt::to_cbor(&contract).expect("contract should be serialized");

    drive
        .apply_contract_cbor(
            contract_cbor,
            contract_id,
            BlockInfo::default(),
            true,
            None,
            transaction,
        )
        .expect("contract should be applied");
    contract
}

/// Serializes to CBOR and applies to Drive a contract from hex string format.
pub fn setup_contract_from_hex(
    drive: &Drive,
    hex_string: String,
    transaction: TransactionArg,
) -> Contract {
    let contract_cbor = cbor_from_hex(hex_string);
    let contract = <Contract as DriveContractExt>::from_cbor(&contract_cbor, None)
        .expect("contract should be deserialized");
    drive
        .apply_contract_cbor(
            contract_cbor,
            None,
            BlockInfo::default(),
            true,
            None,
            transaction,
        )
        .expect("contract should be applied");
    contract
}

/// Serializes a hex string to CBOR.
pub fn cbor_from_hex(hex_string: String) -> Vec<u8> {
    hex::decode(hex_string).expect("Decoding failed")
}

/// Takes a file and returns the lines as a list of strings.
pub fn text_file_strings(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("file not found");
    let reader = io::BufReader::new(file).lines();
    reader.into_iter().map(|a| a.unwrap()).collect()
}