use arraystring::{typenum::U100, ArrayString};
use validator::ValidationError;

pub type Token = ArrayString<U100>;
pub fn validate_token_length(token: &Token) -> Result<(), ValidationError> {
    if token.as_str().len() != 100 {
        return Err(ValidationError::new("Invalid token"));
    }

    Ok(())
}
