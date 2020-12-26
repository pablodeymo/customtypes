use arraystring::{typenum::U255, typenum::U30, ArrayString};
use validator::ValidationError;

#[cfg(feature = "enablediesel")]
#[macro_use]
extern crate validator_derive;
#[cfg(feature = "enablediesel")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "enablediesel")]
pub mod combo;
#[cfg(feature = "enablediesel")]
pub mod countryenum;
#[cfg(feature = "enableactix")]
pub mod httpmsgid;
#[cfg(feature = "enablediesel")]
pub mod langenum;
pub mod password;
#[cfg(feature = "enableactix")]
pub mod responsefind;
pub mod token;

pub type Name = ArrayString<U255>;
pub fn validate_name_length(name: &Name) -> Result<(), ValidationError> {
    if name.as_str().is_empty() {
        return Err(ValidationError::new("Invalid name"));
    }

    Ok(())
}

pub type PhoneNumber = ArrayString<U30>;
