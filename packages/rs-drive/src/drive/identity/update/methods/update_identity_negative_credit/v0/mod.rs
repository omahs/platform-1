use grovedb::Element;
use dpp::state_transition::fee::Credits;
use crate::drive::Drive;
use crate::drive::identity::{identity_path_vec, IdentityRootStructure};
use crate::fee::op::LowLevelDriveOperation;

impl Drive {
    /// We can set an identities negative credit balance
    pub fn update_identity_negative_credit_operation_v0(
        &self,
        identity_id: [u8; 32],
        negative_credit: Credits,
    ) -> LowLevelDriveOperation {
        let identity_path = identity_path_vec(identity_id.as_slice());

        // The value needs to be replaced without changing storage fees so we use bytes instead of varint
        let new_negative_credit_bytes = negative_credit.to_be_bytes().to_vec();

        LowLevelDriveOperation::replace_for_known_path_key_element(
            identity_path,
            vec![IdentityRootStructure::IdentityTreeNegativeCredit as u8],
            Element::new_item(new_negative_credit_bytes),
        )
    }
}