use crate::identity::KeyID;
use crate::state_transition::data_contract_update_transition::DataContractUpdateTransition;
use crate::state_transition::StateTransitionIdentitySigned;

impl StateTransitionIdentitySigned for DataContractUpdateTransition {
    fn get_signature_public_key_id(&self) -> Option<KeyID> {
        match self {
            DataContractUpdateTransition::V0(transition) => {
                transition.get_signature_public_key_id()
            }
        }
    }

    fn set_signature_public_key_id(&mut self, key_id: KeyID) {
        match self {
            DataContractUpdateTransition::V0(transition) => transition.set_signature_public_key_id(key_id),
        }
    }
}