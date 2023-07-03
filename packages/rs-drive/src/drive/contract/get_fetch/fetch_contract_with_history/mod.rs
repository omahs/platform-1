use std::collections::BTreeMap;
use grovedb::TransactionArg;
use dpp::version::drive_versions::DriveVersion;
use crate::drive::Drive;
use crate::error::drive::DriveError;
use crate::error::Error;

mod v0;

impl Drive {
    /// Fetches a contract along with its history.
    ///
    /// # Arguments
    ///
    /// * `contract_id` - A 32-byte array representing the unique identifier of the contract.
    ///
    /// * `transaction` - A transaction that requests the contract.
    ///
    /// * `start_at_date` - A `u64` representing the timestamp in Unix Epoch format from which to
    /// start fetching the contract's history.
    ///
    /// * `limit` - An `Option<u16>` that sets the maximum number of contract history entries
    /// to return. If `None`, the limit is set to 10. Should be between 1 and 10.
    ///
    /// * `offset` - An `Option<u16>` that sets the number of contract history entries to skip
    /// before starting to return them. If `None`, no entries are skipped.
    ///
    /// * `drive_version` - The version of the drive used to select the correct method version.
    ///
    /// # Returns
    ///
    /// * `Result<BTreeMap<u64, Contract>, Error>` - A `Result` type, where `Ok` variant contains
    /// a `BTreeMap` with Unix timestamp as the key and contract as the value, representing
    /// the contract's history. The `Err` variant contains an `Error` in case of a failure.
    ///
    /// # Errors
    ///
    /// This function will return an `Error` in the following situations:
    ///
    /// * If the drive version does not match any of the implemented method versions.
    ///
    /// * If any of the parameters are invalid for querying contract history.
    ///
    /// * If the contract cannot be deserialized due to protocol errors.
    ///
    /// * If the queried contract path does not refer to a contract element.
    pub fn fetch_contract_with_history(
        &self,
        contract_id: [u8; 32],
        transaction: TransactionArg,
        start_at_date: u64,
        limit: Option<u16>,
        offset: Option<u16>,
        drive_version: &DriveVersion,
    ) -> Result<BTreeMap<u64, Contract>, Error> {
        match drive_version.methods.contract.get.fetch_contract_with_history {
            0 => self.fetch_contract_with_history_v0(contract_id, transaction, start_at_date, limit, offset, drive_version),
            version => Err(Error::Drive(DriveError::UnknownVersionMismatch {
                method: "fetch_contract_with_history".to_string(),
                known_versions: vec![0],
                received: version,
            })),
        }
    }
}


#[cfg(feature = "full")]
#[cfg(test)]
mod tests {

        use super::*;
        use crate::error::drive::DriveError;
        use crate::error::Error;
        use dpp::block::extended_block_info::BlockInfo;
        use dpp::data_contract::DataContract;
        use dpp::tests::fixtures::get_data_contract_fixture;
        use serde_json::json;
    use crate::tests::helpers::setup::setup_drive_with_initial_state_structure;

    struct TestData {
            data_contract: DataContract,
            drive: Drive,
        }

        fn apply_contract(drive: &Drive, data_contract: &DataContract, block_info: BlockInfo) {
            let drive_version = DriveVersion::latest();
            drive
                .apply_contract(data_contract, block_info, true, None, None, &drive_version)
                .expect("to apply contract");
        }

        fn insert_n_contract_updates(data_contract: &DataContract, drive: &Drive, n: u64) {
            let updated_document_template = json!({
                "type": "object",
                "properties": {
                    "name": {
                        "type": "string"
                    },
                    "newProp": {
                        "type": "integer",
                        "minimum": 0
                    }
                },
                "required": [
                "$createdAt"
                ],
                "additionalProperties": false
            });

            let mut data_contract = data_contract.clone();
            for i in 0..n {
                let mut updated_document = updated_document_template.clone();
                updated_document
                    .as_object_mut()
                    .expect("document to be an object")
                    .get_mut("properties")
                    .expect("properties to be present")
                    .as_object_mut()
                    .expect("properties to be an object")
                    .insert(
                        format!("newProp{}", i),
                        json!({"type": "integer", "minimum": 0}),
                    );

                data_contract
                    .set_document_schema("niceDocument".into(), updated_document)
                    .expect("to be able to set document schema");
                data_contract.increment_version();

                apply_contract(
                    drive,
                    &data_contract,
                    BlockInfo {
                        time_ms: 1000 * (i + 2),
                        height: 100 + i,
                        core_height: (10 + i) as u32,
                        epoch: Default::default(),
                    },
                );
            }
        }

        pub fn setup_history_test_with_n_updates(
            mut data_contract: DataContract,
            drive: &Drive,
            n: u64,
        ) -> DataContract {
            data_contract.config.keeps_history = true;
            data_contract.config.readonly = false;

            let original_data_contract = data_contract.clone();

            apply_contract(
                drive,
                &data_contract,
                BlockInfo {
                    time_ms: 1000,
                    height: 100,
                    core_height: 10,
                    epoch: Default::default(),
                },
            );

            insert_n_contract_updates(&data_contract, drive, n);

            original_data_contract
        }

        pub fn assert_property_exists(data_contract: &DataContract, property: &str) {
            let updated_document = data_contract
                .get_document_schema("niceDocument")
                .expect("to get document schema");
            let updated_document = updated_document.as_object().expect("to be an object");
            let properties = updated_document
                .get("properties")
                .expect("to have properties")
                .as_object()
                .expect("properties to be an object");

            let property_keys = properties
                .keys()
                .map(|key| key.to_string())
                .collect::<Vec<String>>();

            assert!(
                properties.contains_key(property),
                "expect property {} to exist. Instead found properties {:?}",
                property,
                property_keys
            );
        }

        fn setup_test() -> TestData {
            let data_contract = get_data_contract_fixture(None).data_contract;

            TestData {
                data_contract,
                drive: setup_drive_with_initial_state_structure(),
            }
        }

        #[test]
        pub fn should_fetch_10_latest_contract_without_offset_and_limit_and_start_date_0() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 0,
                limit: None,
                offset: None,
                expected_length: 10,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                // Contract created at 1000, 20 updates applied. The last update is at 21000
                // The 5th update from the latest update is 21000 - 10000 = 11000, plus since
                // the latest update is included into result, the expected oldest update date
                // is 12000.
                expected_oldest_update_date_in_result_ms: 12000,
                // 10th oldest update after 20 is 10.
                expected_oldest_update_index_in_result: 10,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_with_limit_without_offset() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 0,
                limit: Some(5),
                offset: None,
                expected_length: 5,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 17000,
                expected_oldest_update_index_in_result: 15,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_without_limit_with_offset() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 0,
                limit: None,
                offset: Some(5),
                expected_length: 10,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                // Same as test case above, but with offset 5
                expected_oldest_update_date_in_result_ms: 7000,
                expected_oldest_update_index_in_result: 5,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_with_limit_with_offset() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 0,
                limit: Some(5),
                offset: Some(5),
                expected_length: 5,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 12000,
                expected_oldest_update_index_in_result: 10,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_with_non_zero_start_date() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 5000,
                limit: None,
                offset: None,
                expected_length: 10,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 12000,
                expected_oldest_update_index_in_result: 10,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fail_with_limit_higher_than_10() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 5000,
                limit: Some(11),
                offset: None,
                expected_length: 0,
                expected_error: Some(Error::Drive(DriveError::InvalidContractHistoryFetchLimit(
                    11,
                ))),
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 0,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fail_with_limit_smaller_than_1() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 5000,
                limit: Some(0),
                offset: None,
                expected_length: 0,
                expected_error: Some(Error::Drive(DriveError::InvalidContractHistoryFetchLimit(
                    0,
                ))),
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 0,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_empty_with_start_date_after_latest_update() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 21001,
                limit: None,
                offset: None,
                expected_length: 0,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 0,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_return_empty_result_with_non_existent_contract_id() {
            let test_case = TestCase {
                total_updates_to_apply: 20,
                start_at_date: 5000,
                limit: None,
                offset: None,
                expected_length: 0,
                expected_error: None,
                query_non_existent_contract_id: true,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 0,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_only_oldest_updates_with_offset_regardless_of_limit_when_not_enough_updates(
        ) {
            let test_case = TestCase {
                total_updates_to_apply: 15,
                start_at_date: 0,
                limit: Some(10),
                offset: Some(10),
                // 5 updates and the original contract
                expected_length: 6,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                // The same as created date, since we only have 5 updates with such offset
                expected_oldest_update_date_in_result_ms: 1000,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: true,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_empty_history_when_offset_is_so_large_that_no_updates_can_be_fetched() {
            let test_case = TestCase {
                total_updates_to_apply: 15,
                start_at_date: 0,
                limit: Some(10),
                offset: Some(20),
                // With offset being larger than total updates, we should offset - total_updates
                // results, even if limit is set to 10
                expected_length: 0,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 0,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_with_limit_equals_total_updates() {
            let test_case = TestCase {
                total_updates_to_apply: 10,
                start_at_date: 0,
                limit: Some(10), // limit equals to total updates
                offset: None,
                expected_length: 10, // still should return 10 due to the constraint of maximum 10 results
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 2000,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_only_latest_updates_if_updates_count_lower_than_the_limit() {
            let test_case = TestCase {
                total_updates_to_apply: 7,
                start_at_date: 0,
                limit: Some(10), // limit larger than total updates
                offset: None,
                expected_length: 8,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 1000,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: true,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_handle_when_no_updates_at_all() {
            let test_case = TestCase {
                total_updates_to_apply: 0,
                start_at_date: 0,
                limit: None,
                offset: None,
                expected_length: 1,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 1000,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: true,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_empty_when_start_date_is_in_future() {
            let test_case = TestCase {
                total_updates_to_apply: 10,
                start_at_date: 20000, // future date
                limit: None,
                offset: None,
                expected_length: 0,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 0,
                expected_oldest_update_index_in_result: 0,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        #[test]
        pub fn should_fetch_when_start_date_is_same_as_latest_update() {
            let test_case = TestCase {
                total_updates_to_apply: 10,
                // TODO: important! This date is exclusive, that's why we can't query
                //  with the same date as the latest update. Check if this is the correct
                //  behavior
                start_at_date: 10999,
                limit: None,
                offset: None,
                expected_length: 1,
                expected_error: None,
                query_non_existent_contract_id: false,
                contract_created_date_ms: 1000,
                update_period_interval_ms: 1000,
                expected_oldest_update_date_in_result_ms: 11000,
                expected_oldest_update_index_in_result: 9,
                expect_result_to_include_original_contract: false,
            };

            run_single_test_case(test_case);
        }

        struct TestCase {
            // Test set up parameters
            total_updates_to_apply: usize,
            contract_created_date_ms: u64,
            update_period_interval_ms: u64,

            // The query parameters
            start_at_date: u64,
            limit: Option<u16>,
            offset: Option<u16>,
            query_non_existent_contract_id: bool,

            // Expected outcomes
            expected_length: usize,
            expected_error: Option<Error>,
            expected_oldest_update_date_in_result_ms: u64,
            // The index of the oldest update in the result. So if we expect the oldest result
            // to be 10th update, then this value should be 9, because the index starts from 0
            // and not 1. It is used to generate property names in the updated contract, so we
            // can verify that the result is correct.
            expected_oldest_update_index_in_result: u64,

            expect_result_to_include_original_contract: bool,
        }

        fn run_single_test_case(test_case: TestCase) {
            let TestData {
                data_contract,
                drive,
            } = setup_test();

            let contract_id = if test_case.query_non_existent_contract_id {
                [0u8; 32]
            } else {
                *data_contract.id().as_bytes()
            };
            let original_data_contract = setup_history_test_with_n_updates(
                data_contract,
                &drive,
                test_case.total_updates_to_apply as u64,
            );

            let contract_history_result = drive.fetch_contract_with_history(
                contract_id,
                None,
                test_case.start_at_date,
                test_case.limit,
                test_case.offset,
                drive_version,
            );

            match &test_case.expected_error {
                Some(expected_error) => {
                    assert!(contract_history_result.is_err());
                    // Error doesn't implement PartialEq, so we have to compare the strings
                    assert_eq!(
                        contract_history_result.unwrap_err().to_string(),
                        expected_error.to_string()
                    );
                }
                None => {
                    assert!(contract_history_result.is_ok());
                    let contract_history = contract_history_result.unwrap();
                    assert_eq!(contract_history.len(), test_case.expected_length);

                    for (i, (key, contract)) in contract_history.iter().enumerate() {
                        if i == 0 && test_case.expect_result_to_include_original_contract {
                            // TODO: this doesn't work because when we deserialize the contract
                            //  keeps_history is false for some reason!
                            assert_eq!(key, &test_case.contract_created_date_ms);
                            assert_eq!(contract, &original_data_contract);
                            continue;
                        }

                        let expected_key: u64 = test_case.expected_oldest_update_date_in_result_ms
                            + i as u64 * test_case.update_period_interval_ms;
                        assert_eq!(key, &expected_key);

                        let prop_index = if test_case.expect_result_to_include_original_contract {
                            // If we expect the result to include the original contract, then
                            // the first update will be the original contract, so we need to
                            // offset the index by 1
                            i - 1 + test_case.expected_oldest_update_index_in_result as usize
                        } else {
                            i + test_case.expected_oldest_update_index_in_result as usize
                        };

                        // When updating a contract, we add a new property to it
                        // TODO: this test actually applies incompatible updates to the contract
                        //  because we don't validate the contract in the apply function
                        assert_property_exists(contract, format!("newProp{}", prop_index).as_str());
                    }
                }
        }
    }
}