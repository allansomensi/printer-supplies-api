use axum::{extract::State, response::IntoResponse, Json};
use sqlx::migrate;
use std::sync::Arc;
use tracing::{error, info};

use crate::models::database::AppState;

pub async fn dry_run() -> impl IntoResponse {
    todo!("Dry run mode is planned but has not been implemented yet.");
}

pub async fn live_run(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match migrate!("./migrations").run(&state.db).await {
        Ok(_) => {
            info!("Migrations applied successfully!");
            Json(String::from("Migrations applied successfully!"))
        }
        Err(e) => {
            error!("Error applying migrations: {e}");
            Json(String::from("Error applying migrations."))
        }
    }
}
