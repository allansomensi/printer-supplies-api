use super::{
    brand::Brand,
    supplies::{drum::Drum, toner::Toner},
};
use crate::validations::uuid::is_uuid;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Printer {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub brand: Uuid,
    pub toner: Uuid,
    pub drum: Uuid,
}

impl Printer {
    pub fn new(name: &str, model: &str, brand: Uuid, toner: Uuid, drum: Uuid) -> Self {
        Printer {
            id: Uuid::new_v4(),
            name: String::from(name),
            model: String::from(model),
            brand,
            toner,
            drum,
        }
    }
}

pub type PrinterView = (
    Uuid,            // printer_id
    String,          // printer_name
    String,          // printer_model
    Uuid,            // brand_id
    String,          // brand_name
    Uuid,            // toner_id
    String,          // toner_name
    Option<i32>,     // toner_stock
    Option<Decimal>, // toner_price
    Uuid,            // drum_id
    String,          // drum_name
    Option<i32>,     // drum_stock
    Option<Decimal>, // drum_price
);

#[derive(Serialize, ToSchema)]
pub struct PrinterDetails {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub brand: Brand,
    pub toner: Toner,
    pub drum: Drum,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct CreatePrinterRequest {
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: String,
    #[validate(length(min = 3, message = "Model must be greater than 3 chars"))]
    pub model: String,
    #[validate(custom(function = "is_uuid"))]
    pub brand: String,
    #[validate(custom(function = "is_uuid"))]
    pub toner: String,
    #[validate(custom(function = "is_uuid"))]
    pub drum: String,
}

#[derive(Deserialize, Serialize, ToSchema, Validate)]
pub struct UpdatePrinterRequest {
    pub id: Uuid,
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub name: Option<String>,
    #[validate(length(min = 3, message = "Model must be greater than 3 chars"))]
    pub model: Option<String>,
    #[validate(custom(function = "is_uuid"))]
    pub brand: Option<String>,
    #[validate(custom(function = "is_uuid"))]
    pub toner: Option<String>,
    #[validate(custom(function = "is_uuid"))]
    pub drum: Option<String>,
}
