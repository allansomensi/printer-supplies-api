use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub mod brand;
pub mod database;
pub mod movement;
pub mod printer;
pub mod status;
pub mod supplies;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct DeleteRequest {
    pub id: Uuid,
}
