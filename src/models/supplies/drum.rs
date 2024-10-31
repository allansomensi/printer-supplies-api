use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, FromRow, ToSchema)]
pub struct Drum {
    pub id: Uuid,
    pub name: String,
    pub stock: Option<i32>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub price: Option<Decimal>,
}

impl Default for Drum {
    fn default() -> Self {
        Drum {
            id: Uuid::new_v4(),
            name: String::from("Unknown"),
            stock: None,
            price: None,
        }
    }
}

impl Drum {
    pub fn new(name: &str, stock: Option<i32>, price: Option<Decimal>) -> Self {
        Drum {
            id: Uuid::new_v4(),
            name: String::from(name),
            stock,
            price,
        }
    }
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreateDrumRequest {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(range(min = 0, message = "Stock must be greater or equal than 0"))]
    pub stock: Option<i32>,
    pub price: Option<Decimal>,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdateDrumRequest {
    pub id: Uuid,
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: Option<String>,
    #[validate(range(min = 0, message = "Stock must be greater or equal than 0"))]
    pub stock: Option<i32>,
    pub price: Option<Decimal>,
}
