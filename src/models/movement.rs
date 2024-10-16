use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

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

#[derive(Serialize)]
pub struct MovementDetails {
    pub id: Uuid,
    pub printer: PrinterDetails,
    pub item: ItemDetails,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ItemDetails {
    pub id: Uuid,
    pub name: String,
    pub stock: i32,
}

#[derive(Serialize)]
pub struct PrinterDetails {
    pub id: Uuid,
    pub name: String,
    pub model: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct CreateMovementRequest {
    pub printer_id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct UpdateMovementRequest {
    pub id: Uuid,
    pub printer_id: Uuid,
    pub item_id: Uuid,
    pub quantity: i32,
}
