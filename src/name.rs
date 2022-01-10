use validator::ValidationError;
use arraystring::{typenum::U255, ArrayString};
#[cfg(feature = "enablecalamine")]
use calamine::DataType;

pub type Name = ArrayString<U255>;
pub fn validate_name_length(name: &Name) -> Result<(), ValidationError> {
    if name.as_str().is_empty() {
        return Err(ValidationError::new("Invalid name"));
    }

    Ok(())
}

#[cfg(feature = "enablecalamine")]
pub fn name_into_datatype(name: &Name) -> DataType {
    DataType::String(String::from(name.as_str()))
}
