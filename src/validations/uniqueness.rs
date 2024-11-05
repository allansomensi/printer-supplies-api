use crate::errors::api_error::ApiError;
use infra::database::AppState;
use std::sync::Arc;
use tracing::error;

pub async fn is_toner_unique(state: Arc<AppState>, toner_name: String) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM toners WHERE name = $1;"#)
        .bind(&toner_name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking for existing toner: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if exists {
        error!("Toner '{}' already exists.", &toner_name);
        Err(ApiError::AlreadyExists)
    } else {
        Ok(())
    }
}

pub async fn is_drum_unique(state: Arc<AppState>, drum_name: String) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM drums WHERE name = $1;"#)
        .bind(&drum_name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking for existing drum: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if exists {
        error!("Drum '{}' already exists.", &drum_name);
        Err(ApiError::AlreadyExists)
    } else {
        Ok(())
    }
}

pub async fn is_brand_unique(state: Arc<AppState>, brand_name: String) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM brands WHERE name = $1;"#)
        .bind(&brand_name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking for existing brand: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if exists {
        error!("Brand '{}' already exists.", &brand_name);
        Err(ApiError::AlreadyExists)
    } else {
        Ok(())
    }
}

pub async fn is_printer_unique(state: Arc<AppState>, printer_name: String) -> Result<(), ApiError> {
    let exists = sqlx::query(r#"SELECT id FROM printers WHERE name = $1;"#)
        .bind(&printer_name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking for existing printer: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if exists {
        error!("Printer '{}' already exists.", &printer_name);
        Err(ApiError::AlreadyExists)
    } else {
        Ok(())
    }
}
