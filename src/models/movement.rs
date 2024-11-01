use crate::validations::uuid::is_uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Serialize, FromRow)]
pub struct Movement {
    pub id: Uuid,
    pub printer_id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

impl Movement {
    pub fn new(printer_id: Uuid, item_id: Uuid, quantity: i32) -> Self {
        Self {
            id: Uuid::new_v4(),
            printer_id,
            item_id,
            quantity,
            created_at: Utc::now(),
        }
    }
}

pub type MovementView = (
    Uuid,          // movement_id
    Uuid,          // printer_id
    String,        // printer_name
    String,        // printer_model
    Uuid,          // item_id
    String,        // item_name
    i32,           // quantity
    DateTime<Utc>, // created_at
);

#[derive(Serialize, ToSchema)]
pub struct MovementDetails {
    pub id: Uuid,
    pub printer: PrinterDetails,
    pub item: ItemDetails,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct ItemDetails {
    pub id: Uuid,
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct PrinterDetails {
    pub id: Uuid,
    pub name: String,
    pub model: String,
}

#[derive(Deserialize, Serialize, FromRow, ToSchema, Validate)]
pub struct CreateMovementRequest {
    #[validate(custom(function = "is_uuid"))]
    pub printer_id: String,
    #[validate(custom(function = "is_uuid"))]
    pub item_id: String,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, FromRow, ToSchema, Validate)]
pub struct UpdateMovementRequest {
    #[validate(custom(function = "is_uuid"))]
    pub id: String,
    #[validate(custom(function = "is_uuid"))]
    pub printer_id: Option<String>,
    #[validate(custom(function = "is_uuid"))]
    pub item_id: Option<String>,
    pub quantity: Option<i32>,
}
