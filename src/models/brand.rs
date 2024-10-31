use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, FromRow, ToSchema)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
}

impl Brand {
    pub fn new(name: &str) -> Self {
        Brand {
            id: Uuid::new_v4(),
            name: String::from(name),
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateBrandRequest {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateBrandRequest {
    pub id: Uuid,
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
}
