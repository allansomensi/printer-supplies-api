use std::fmt::{Display, Formatter};

use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Toner {
    id: Uuid,
    name: TonerName,
}

impl Toner {
    pub fn new(id: Uuid, name: TonerName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn name(&self) -> &TonerName {
        &self.name
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TonerName(String);

#[derive(Clone, Debug, Error)]
#[error("Toner name cannot be empty")]
pub struct TonerNameEmptyError;

impl TonerName {
    pub fn new(raw: &str) -> Result<Self, TonerNameEmptyError> {
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            Err(TonerNameEmptyError)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }
}

impl Display for TonerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTonerRequest {
    name: TonerName,
}

impl CreateTonerRequest {
    pub fn new(name: TonerName) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &TonerName {
        &self.name
    }
}

#[derive(Debug, Error)]
pub enum CreateTonerError {
    #[error("Toner with name {name} already exists")]
    Duplicate { name: TonerName },
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
