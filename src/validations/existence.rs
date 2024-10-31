use crate::{database::AppState, errors::api_error::ApiError};
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

pub async fn toner_exists(state: Arc<AppState>, toner_id: Uuid) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(toner_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error fetching toner by ID: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if !exists {
        error!("Toner ID not found.");
        Err(ApiError::IdNotFound)
    } else {
        Ok(())
    }
}

pub async fn drum_exists(state: Arc<AppState>, drum_id: Uuid) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM drums WHERE id = $1;"#)
        .bind(drum_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error fetching drum by ID: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if !exists {
        error!("Drum ID not found.");
        Err(ApiError::IdNotFound)
    } else {
        Ok(())
    }
}

pub async fn brand_exists(state: Arc<AppState>, brand_id: Uuid) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM brands WHERE id = $1;"#)
        .bind(brand_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error fetching brand by ID: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if !exists {
        error!("Brand ID not found.");
        Err(ApiError::IdNotFound)
    } else {
        Ok(())
    }
}
