use crate::{
    errors::api_error::ApiError,
    models::status::{Database, Dependencies, Status},
};
use axum::{extract::State, response::IntoResponse, Json};
use chrono::Utc;
use infra::database::AppState;
use std::{env, sync::Arc};
use tracing::{error, info};

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
pub async fn show_status(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let version = sqlx::query_scalar::<_, String>(r#"SHOW server_version;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving database version: {e}");
            ApiError::DatabaseError(e)
        })?;

    let max_connections = sqlx::query_scalar::<_, String>(r#"SHOW max_connections;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving database max connections: {e}");
            ApiError::DatabaseError(e)
        })?
        .parse::<i64>()
        .expect("Error parsing max_connections as i64");

    let opened_connections = sqlx::query_scalar::<_, i64>(
        r#"SELECT count(*) FROM pg_stat_activity WHERE datname = $1;"#,
    )
    .bind(env::var("POSTGRES_DB").unwrap())
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("Error retrieving database opened connections: {e}");
        ApiError::DatabaseError(e)
    })?;

    let database = Database {
        version,
        max_connections,
        opened_connections,
    };

    info!("Status queried");
    Ok(Json(Status {
        updated_at: Utc::now(),
        dependencies: Dependencies { database },
    }))
}
