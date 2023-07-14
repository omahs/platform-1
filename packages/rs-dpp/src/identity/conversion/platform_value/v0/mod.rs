use platform_value::Value;
use crate::ProtocolError;
use crate::version::PlatformVersion;

pub trait IdentityPlatformValueConversionMethodsV0 {
    fn to_object(&self) -> Result<Value, ProtocolError>;
    fn into_object(self) -> Result<Value, ProtocolError>;
    fn to_cleaned_object(&self) -> Result<Value, ProtocolError>;
    /// Creates an identity from a raw object
    fn from_object(raw_object: Value, platform_version: &PlatformVersion) -> Result<Self, ProtocolError> where Self: Sized;
}