use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::consensus::basic::data_contract::data_contract_max_depth_exceed_error::DataContractMaxDepthExceedError;
use crate::consensus::basic::data_contract::{
    DataContractHaveNewUniqueIndexError, DataContractImmutablePropertiesUpdateError,
    DataContractInvalidIndexDefinitionUpdateError, DataContractUniqueIndicesChangedError,
    DuplicateIndexError, DuplicateIndexNameError, IncompatibleDataContractSchemaError,
    IncompatibleRe2PatternError, InvalidCompoundIndexError, InvalidDataContractIdError,
    InvalidDataContractVersionError, InvalidIndexPropertyTypeError,
    InvalidIndexedPropertyConstraintError, InvalidJsonSchemaRefError,
    SystemPropertyIndexAlreadyPresentError, UndefinedIndexPropertyError,
    UniqueIndicesLimitReachedError,
};
use crate::consensus::basic::decode::{ProtocolVersionParsingError, SerializedObjectParsingError};
use crate::consensus::basic::document::{
    DataContractNotPresentError, DuplicateDocumentTransitionsWithIdsError,
    DuplicateDocumentTransitionsWithIndicesError, InconsistentCompoundIndexDataError,
    InvalidDocumentTransitionActionError, InvalidDocumentTransitionIdError,
    InvalidDocumentTypeError, MissingDataContractIdBasicError,
    MissingDocumentTransitionActionError, MissingDocumentTransitionTypeError,
    MissingDocumentTypeError,
};
use crate::consensus::basic::identity::{
    DuplicatedIdentityPublicKeyBasicError, DuplicatedIdentityPublicKeyIdBasicError,
    IdentityAssetLockProofLockedTransactionMismatchError,
    IdentityAssetLockTransactionIsNotFoundError,
    IdentityAssetLockTransactionOutPointAlreadyExistsError,
    IdentityAssetLockTransactionOutputNotFoundError, InvalidAssetLockProofCoreChainHeightError,
    InvalidAssetLockProofTransactionHeightError, InvalidAssetLockTransactionOutputReturnSizeError,
    InvalidIdentityAssetLockProofChainLockValidationError,
    InvalidIdentityAssetLockTransactionError, InvalidIdentityAssetLockTransactionOutputError,
    InvalidIdentityCreditWithdrawalTransitionCoreFeeError,
    InvalidIdentityCreditWithdrawalTransitionOutputScriptError, InvalidIdentityKeySignatureError,
    InvalidIdentityPublicKeyDataError, InvalidIdentityPublicKeySecurityLevelError,
    InvalidInstantAssetLockProofError, InvalidInstantAssetLockProofSignatureError,
    MissingMasterPublicKeyError, NotImplementedIdentityCreditWithdrawalTransitionPoolingError,
};
use crate::consensus::basic::invalid_identifier_error::InvalidIdentifierError;
use crate::consensus::basic::state_transition::{
    InvalidStateTransitionTypeError, MissingStateTransitionTypeError,
    StateTransitionMaxSizeExceededError,
};
use crate::consensus::basic::{IncompatibleProtocolVersionError, UnsupportedProtocolVersionError};
use crate::consensus::ConsensusError;

use crate::consensus::basic::json_schema_compilation_error::JsonSchemaCompilationError;
use crate::consensus::basic::json_schema_error::JsonSchemaError;
use crate::consensus::basic::value_error::ValueError;

#[derive(Error, Debug, Serialize, Deserialize, Encode, Decode)]
pub enum BasicError {
    /*

    DO NOT CHANGE ORDER OF VARIANTS WITHOUT INTRODUCING OF NEW VERSION

    */
    // Decoding
    #[error(transparent)]
    ProtocolVersionParsingError(ProtocolVersionParsingError),

    #[error(transparent)]
    SerializedObjectParsingError(SerializedObjectParsingError),

    #[error(transparent)]
    UnsupportedProtocolVersionError(UnsupportedProtocolVersionError),

    #[error(transparent)]
    IncompatibleProtocolVersionError(IncompatibleProtocolVersionError),

    // Structure error
    #[error(transparent)]
    JsonSchemaCompilationError(JsonSchemaCompilationError),

    #[error(transparent)]
    JsonSchemaError(JsonSchemaError),

    #[error(transparent)]
    InvalidIdentifierError(InvalidIdentifierError),

    #[error(transparent)]
    ValueError(ValueError),

    // DataContract
    #[error(transparent)]
    DataContractMaxDepthExceedError(DataContractMaxDepthExceedError),

    #[error(transparent)]
    DuplicateIndexError(DuplicateIndexError),

    #[error(transparent)]
    IncompatibleRe2PatternError(IncompatibleRe2PatternError),

    #[error(transparent)]
    InvalidCompoundIndexError(InvalidCompoundIndexError),

    #[error(transparent)]
    InvalidDataContractIdError(InvalidDataContractIdError),

    #[error(transparent)]
    InvalidIndexedPropertyConstraintError(InvalidIndexedPropertyConstraintError),

    #[error(transparent)]
    InvalidIndexPropertyTypeError(InvalidIndexPropertyTypeError),

    #[error(transparent)]
    InvalidJsonSchemaRefError(InvalidJsonSchemaRefError),

    #[error(transparent)]
    SystemPropertyIndexAlreadyPresentError(SystemPropertyIndexAlreadyPresentError),

    #[error(transparent)]
    UndefinedIndexPropertyError(UndefinedIndexPropertyError),

    #[error(transparent)]
    UniqueIndicesLimitReachedError(UniqueIndicesLimitReachedError),

    #[error(transparent)]
    DuplicateIndexNameError(DuplicateIndexNameError),

    #[error(transparent)]
    InvalidDataContractVersionError(InvalidDataContractVersionError),

    #[error(transparent)]
    IncompatibleDataContractSchemaError(IncompatibleDataContractSchemaError),

    #[error(transparent)]
    DataContractImmutablePropertiesUpdateError(DataContractImmutablePropertiesUpdateError),

    #[error(transparent)]
    DataContractUniqueIndicesChangedError(DataContractUniqueIndicesChangedError),

    #[error(transparent)]
    DataContractInvalidIndexDefinitionUpdateError(DataContractInvalidIndexDefinitionUpdateError),

    #[error(transparent)]
    DataContractHaveNewUniqueIndexError(DataContractHaveNewUniqueIndexError),

    // Document
    #[error(transparent)]
    DataContractNotPresentError(DataContractNotPresentError),

    #[error(transparent)]
    DuplicateDocumentTransitionsWithIdsError(DuplicateDocumentTransitionsWithIdsError),

    #[error(transparent)]
    DuplicateDocumentTransitionsWithIndicesError(DuplicateDocumentTransitionsWithIndicesError),

    #[error(transparent)]
    InconsistentCompoundIndexDataError(InconsistentCompoundIndexDataError),

    #[error(transparent)]
    InvalidDocumentTransitionActionError(InvalidDocumentTransitionActionError),

    #[error(transparent)]
    InvalidDocumentTransitionIdError(InvalidDocumentTransitionIdError),

    #[error(transparent)]
    InvalidDocumentTypeError(InvalidDocumentTypeError),

    #[error(transparent)]
    MissingDataContractIdBasicError(MissingDataContractIdBasicError),

    #[error(transparent)]
    MissingDocumentTransitionActionError(MissingDocumentTransitionActionError),

    #[error(transparent)]
    MissingDocumentTransitionTypeError(MissingDocumentTransitionTypeError),

    #[error(transparent)]
    MissingDocumentTypeError(MissingDocumentTypeError),

    // Identity
    #[error(transparent)]
    DuplicatedIdentityPublicKeyBasicError(DuplicatedIdentityPublicKeyBasicError),

    #[error(transparent)]
    DuplicatedIdentityPublicKeyIdBasicError(DuplicatedIdentityPublicKeyIdBasicError),

    #[error(transparent)]
    IdentityAssetLockProofLockedTransactionMismatchError(
        IdentityAssetLockProofLockedTransactionMismatchError,
    ),

    #[error(transparent)]
    IdentityAssetLockTransactionIsNotFoundError(IdentityAssetLockTransactionIsNotFoundError),

    #[error(transparent)]
    IdentityAssetLockTransactionOutPointAlreadyExistsError(
        IdentityAssetLockTransactionOutPointAlreadyExistsError,
    ),

    #[error(transparent)]
    IdentityAssetLockTransactionOutputNotFoundError(
        IdentityAssetLockTransactionOutputNotFoundError,
    ),

    #[error(transparent)]
    InvalidAssetLockProofCoreChainHeightError(InvalidAssetLockProofCoreChainHeightError),

    #[error(transparent)]
    InvalidIdentityAssetLockProofChainLockValidationError(
        InvalidIdentityAssetLockProofChainLockValidationError,
    ),

    #[error(transparent)]
    InvalidAssetLockProofTransactionHeightError(InvalidAssetLockProofTransactionHeightError),

    #[error(transparent)]
    InvalidAssetLockTransactionOutputReturnSizeError(
        InvalidAssetLockTransactionOutputReturnSizeError,
    ),

    #[error(transparent)]
    InvalidIdentityAssetLockTransactionError(InvalidIdentityAssetLockTransactionError),

    #[error(transparent)]
    InvalidIdentityAssetLockTransactionOutputError(InvalidIdentityAssetLockTransactionOutputError),

    #[error(transparent)]
    InvalidIdentityPublicKeyDataError(InvalidIdentityPublicKeyDataError),

    #[error(transparent)]
    InvalidInstantAssetLockProofError(InvalidInstantAssetLockProofError),

    #[error(transparent)]
    InvalidInstantAssetLockProofSignatureError(InvalidInstantAssetLockProofSignatureError),

    #[error(transparent)]
    MissingMasterPublicKeyError(MissingMasterPublicKeyError),

    #[error(transparent)]
    InvalidIdentityPublicKeySecurityLevelError(InvalidIdentityPublicKeySecurityLevelError),

    #[error(transparent)]
    InvalidIdentityKeySignatureError(InvalidIdentityKeySignatureError),

    #[error(transparent)]
    InvalidIdentityCreditWithdrawalTransitionOutputScriptError(
        InvalidIdentityCreditWithdrawalTransitionOutputScriptError,
    ),

    #[error(transparent)]
    InvalidIdentityCreditWithdrawalTransitionCoreFeeError(
        InvalidIdentityCreditWithdrawalTransitionCoreFeeError,
    ),

    #[error(transparent)]
    NotImplementedIdentityCreditWithdrawalTransitionPoolingError(
        NotImplementedIdentityCreditWithdrawalTransitionPoolingError,
    ),

    // State Transition
    #[error(transparent)]
    InvalidStateTransitionTypeError(InvalidStateTransitionTypeError),

    #[error(transparent)]
    MissingStateTransitionTypeError(MissingStateTransitionTypeError),

    #[error(transparent)]
    StateTransitionMaxSizeExceededError(StateTransitionMaxSizeExceededError),
}

impl From<BasicError> for ConsensusError {
    fn from(error: BasicError) -> Self {
        Self::BasicError(error)
    }
}
