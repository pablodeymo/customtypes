use arraystring::{typenum::U255, ArrayString};
use validator::ValidationError;

#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate diesel;

pub mod combo;
pub mod httpmsgid;
pub mod httpresponsefind;

pub type Name = ArrayString<U255>;
pub fn validate_name_length(name: &Name) -> Result<(), ValidationError> {
    if name.as_str().is_empty() {
        return Err(ValidationError::new("Invalid name"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
