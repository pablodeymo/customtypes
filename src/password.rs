use arraystring::{typenum::U255, ArrayString};
use validator::ValidationError;

pub type Password = ArrayString<U255>;
pub fn validate_password_length(password: &Password) -> Result<(), ValidationError> {
    if password.as_str().len() < 6 {
        return Err(ValidationError::new("Invalid password"));
    }

    Ok(())
}
