use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

type Version = String;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Database {
    pub version: Version,
    pub max_connections: u16,
    pub opened_connections: u16,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Dependencies {
    pub database: Database,
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct Status {
    pub updated_at: DateTime<Utc>,
    pub dependencies: Dependencies,
}
