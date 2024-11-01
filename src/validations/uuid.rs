use std::borrow::Cow;
use uuid::Uuid;
use validator::ValidationError;

pub fn is_uuid(uuid: &str) -> Result<(), ValidationError> {
    Uuid::parse_str(uuid).map_err(|_| {
        ValidationError::new("INVALID_UUID").with_message(Cow::Borrowed("Invalid UUID"))
    })?;
    Ok(())
}
