use crate::errors::api_error::ApiError;
use infra::database::AppState;
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

pub async fn printer_exists(state: Arc<AppState>, printer_id: Uuid) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM printers WHERE id = $1;"#)
        .bind(printer_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error fetching printer by ID: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if !exists {
        error!("Printer ID not found.");
        Err(ApiError::IdNotFound)
    } else {
        Ok(())
    }
}

pub async fn movement_exists(state: Arc<AppState>, movement_id: Uuid) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM movements WHERE id = $1;"#)
        .bind(movement_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error fetching movement by ID: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if !exists {
        error!("Movement ID not found.");
        Err(ApiError::IdNotFound)
    } else {
        Ok(())
    }
}
