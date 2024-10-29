use std::{env, sync::Arc};

use axum::{extract::State, Json};
use chrono::Utc;
use tracing::info;

use crate::models::{
    database::AppState,
    status::{Database, Dependencies, Status},
};

/// Retrieves the current status of the API, including the database connection status.
/// Provides information on the database version, maximum connections, and currently open connections.
/// Useful for health checks and monitoring API dependencies.
#[utoipa::path(
    get,
    path = "/api/v1/status",
    tags = ["Status"],
    summary = "Get API and database status",
    description = "Fetches the current operational status of the API, including database information such as version, max connections, and active connections.",
    responses(
        (status = 200, description = "Status retrieved successfully", body = Status)
    )
)]
pub async fn show_status(State(state): State<Arc<AppState>>) -> Json<Status> {
    let db_version: (String,) = sqlx::query_as(r#"SHOW server_version;"#)
        .fetch_one(&state.db)
        .await
        .unwrap();

    let max_connections: (String,) = sqlx::query_as(r#"SHOW max_connections;"#)
        .fetch_one(&state.db)
        .await
        .unwrap();
    let max_connections: u16 = max_connections.0.parse().unwrap();

    let opened_connections: (i32,) =
        sqlx::query_as(r#"SELECT count(*)::int FROM pg_stat_activity WHERE datname = $1;"#)
            .bind(env::var("POSTGRES_DB").unwrap())
            .fetch_one(&state.db)
            .await
            .unwrap();
    let opened_connections: u16 = opened_connections.0 as u16;

    let database = Database {
        version: db_version.0,
        max_connections,
        opened_connections,
    };

    info!("Status queried");
    Json(Status {
        updated_at: Utc::now(),
        dependencies: Dependencies { database },
    })
}
