use crate::data_contract::DataContract;
use crate::identity::signer::Signer;
use crate::identity::{KeyID, PartialIdentity};
use crate::state_transition::documents_batch_transition::DocumentsBatchTransition;
use crate::version::FeatureVersion;
use crate::ProtocolError;
use platform_value::{Bytes32, Identifier};

impl DocumentsBatchTransitionV0Methods for DocumentsBatchTransition {}