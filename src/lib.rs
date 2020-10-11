use arraystring::{typenum::U255, ArrayString};
use validator::ValidationError;

#[cfg(feature = "enablediesel")]
#[macro_use]
extern crate validator_derive;
#[cfg(feature = "enablediesel")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "enablediesel")]
pub mod combo;
#[cfg(feature = "enableactix")]
pub mod httpmsgid;
#[cfg(feature = "enableactix")]
pub mod responsefind;

pub type Name = ArrayString<U255>;
pub fn validate_name_length(name: &Name) -> Result<(), ValidationError> {
    if name.as_str().is_empty() {
        return Err(ValidationError::new("Invalid name"));
    }

    Ok(())
}
