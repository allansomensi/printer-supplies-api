use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Toner {
    pub id: Uuid,
    pub name: String,
    pub stock: Option<i32>,
    #[serde(with = "rust_decimal::serde::float_option")]
    pub price: Option<Decimal>,
}

impl Default for Toner {
    fn default() -> Self {
        Toner {
            id: Uuid::new_v4(),
            name: String::from("Unknown"),
            stock: None,
            price: None,
        }
    }
}

impl Toner {
    pub fn new(name: &str, stock: Option<i32>, price: Option<Decimal>) -> Self {
        Toner {
            id: Uuid::new_v4(),
            name: String::from(name),
            stock,
            price,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateTonerRequest {
    pub name: String,
    pub stock: Option<i32>,
    pub price: Option<Decimal>,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTonerRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub stock: Option<i32>,
    pub price: Option<Decimal>,
}
