use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

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

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateDrumRequest {
    pub name: String,
    pub stock: Option<i32>,
    pub price: Option<Decimal>,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdateDrumRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub stock: Option<i32>,
    pub price: Option<Decimal>,
}
