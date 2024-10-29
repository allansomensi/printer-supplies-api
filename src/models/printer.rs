use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{
    brand::Brand,
    supplies::{drum::Drum, toner::Toner},
};

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

#[derive(Serialize, ToSchema)]
pub struct PrinterDetails {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub brand: Brand,
    pub toner: Toner,
    pub drum: Drum,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreatePrinterRequest {
    pub name: String,
    pub model: String,
    pub brand: String,
    pub toner: String,
    pub drum: String,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct UpdatePrinterRequest {
    pub id: Uuid,
    pub name: String,
    pub model: String,
    pub brand: String,
    pub toner: String,
    pub drum: String,
}
