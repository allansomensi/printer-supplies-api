use crate::{
    errors::api_error::ApiError,
    models::{
        supplies::drum::{CreateDrumRequest, Drum, UpdateDrumRequest},
        DeleteRequest,
    },
    validations::{existence::drum_exists, uniqueness::is_drum_unique},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use infra::database::AppState;
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

/// Retrieves the total count of drums.
///
/// This endpoint counts all drums stored in the database and returns the count as an integer.
/// If no drums are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/supplies/drums/count",
    tags = ["Drums"],
    summary = "Get the total count of drums.",
    description = "This endpoint retrieves the total number of drums stored in the database.",
    responses(
        (status = 200, description = "Drum count retrieved successfully", body = i32),
        (status = 500, description = "An error occurred while retrieving the drum count")
    )
)]
pub async fn count_drums(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM drums;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving drum count: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Successfully retrieved drum count: {count}");
    Ok(Json(count))
}

/// Retrieves a specific drum by its ID.
///
/// This endpoint searches for a drum with the specified ID.
/// If the drum is found, it returns the drum details.
#[utoipa::path(
    get,
    path = "/api/v1/supplies/drums/{id}",
    tags = ["Drums"],
    summary = "Get a specific drum by ID.",
    description = "This endpoint retrieves a drum's details from the database using its ID. Returns the drum if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the drum to retrieve", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "Drum retrieved successfully", body = Drum),
        (status = 404, description = "No drum found with the specified ID"),
        (status = 500, description = "An error occurred while retrieving the drum")
    )
)]
pub async fn search_drum(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let drum = sqlx::query_as::<_, Drum>(r#"SELECT * FROM drums WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving drum with id {id}: {e}");
            ApiError::DatabaseError(e)
        })?;

    match drum {
        Some(drum) => {
            info!("Drum found: {id}");
            Ok(Json(drum))
        }
        None => {
            error!("No drum found with id: {id}");
            Err(ApiError::IdNotFound)
        }
    }
}

/// Retrieves a list of all drums.
///
/// This endpoint fetches all drums stored in the database.
/// If there are no drums, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/supplies/drums",
    tags = ["Drums"],
    summary = "List all drums.",
    description = "Fetches all drums stored in the database. If there are no drums, returns an empty array.",
    responses(
        (status = 200, description = "Drums retrieved successfully", body = Vec<Drum>),
        (status = 404, description = "No drums found in the database"),
        (status = 500, description = "An error occurred while retrieving the drums")
    )
)]
pub async fn show_drums(State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, ApiError> {
    let drums = sqlx::query_as::<_, Drum>(r#"SELECT * FROM drums;"#)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Error listing drums: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Drums listed successfully");
    Ok(Json(drums))
}

/// Create a new drum.
///
/// This endpoint creates a new drum by providing its details.
/// Validates the drum's name for length and emptiness, checks for duplicates,
/// and inserts the new drum into the database if all validations pass.
#[utoipa::path(
    post,
    path = "/api/v1/supplies/drums",
    tags = ["Drums"],
    summary = "Create a new drum.",
    description = "This endpoint creates a new drum in the database with the provided details.",
    request_body = CreateDrumRequest,
    responses(
        (status = 201, description = "Drum created successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 409, description = "Conflict: Drum with the same name already exists"),
        (status = 500, description = "An error occurred while creating the drum")
    )
)]
pub async fn create_drum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateDrumRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    is_drum_unique(state.clone(), request.name.clone()).await?;

    let new_drum = Drum::new(&request.name, request.stock, request.price);

    sqlx::query(r#"INSERT INTO drums (id, name, stock, price) VALUES ($1, $2, $3, $4);"#)
        .bind(new_drum.id)
        .bind(&new_drum.name)
        .bind(new_drum.stock)
        .bind(new_drum.price)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error creating drum: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Drum created! ID: {}", &new_drum.id);
    Ok((StatusCode::CREATED, Json(new_drum.id)))
}

/// Updates an existing drum.
///
/// This endpoint updates the details of an existing drum.
/// It accepts the drum ID and the new details for the drum, including its name, stock, and price.
/// The endpoint validates the new name to ensure it is not empty,
/// does not conflict with an existing drum's name, and meets length requirements.
/// If the drum is successfully updated, it returns the UUID of the updated drum.
#[utoipa::path(
    put,
    path = "/api/v1/supplies/drums",
    tags = ["Drums"],
    summary = "Update an existing drum.",
    description = "This endpoint updates the details of an existing drum in the database.",
    request_body = UpdateDrumRequest,
    responses(
        (status = 200, description = "Drum updated successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 404, description = "Drum ID not found"),
        (status = 409, description = "Conflict: Drum with the same name already exists"),
        (status = 500, description = "An error occurred while updating the drum")
    )
)]
pub async fn update_drum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateDrumRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    drum_exists(state.clone(), request.id.clone()).await?;

    let drum_id = request.id;
    let new_name = request.name.clone();
    let new_stock = request.stock;
    let new_price = request.price;

    // Validate and update name if provided
    if let Some(name) = new_name {
        // Update drum name
        sqlx::query(r#"UPDATE drums SET name = $1 WHERE id = $2;"#)
            .bind(&name)
            .bind(drum_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating drum name: {e}");
                ApiError::DatabaseError(e)
            })?;
    }

    // Update stock if provided
    if let Some(stock) = new_stock {
        sqlx::query(r#"UPDATE drums SET stock = $1 WHERE id = $2;"#)
            .bind(stock)
            .bind(drum_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating drum stock: {e}");
                ApiError::DatabaseError(e)
            })?;
    }

    // Update price if provided
    if let Some(price) = new_price {
        sqlx::query(r#"UPDATE drums SET price = $1 WHERE id = $2;"#)
            .bind(price)
            .bind(drum_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating drum price: {e}");
                ApiError::DatabaseError(e)
            })?;
    }

    info!("Drum updated! ID: {}", &drum_id);
    Ok((StatusCode::OK, Json(drum_id)).into_response())
}

/// Deletes an existing drum.
///
/// This endpoint allows users to delete a specific drum by its ID.
/// It checks if the drum exists before attempting to delete it.
/// If the drum is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
    path = "/api/v1/supplies/drums",
    tags = ["Drums"],
    summary = "Delete an existing drum.",
    description = "This endpoint deletes a specific drum from the database using its ID.",
    request_body = DeleteRequest,
    responses(
        (status = 200, description = "Drum deleted successfully", body = String),
        (status = 404, description = "Drum ID not found"),
        (status = 500, description = "An error occurred while deleting the drum")
    )
)]
pub async fn delete_drum(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    drum_exists(state.clone(), request.id.clone()).await?;

    // Delete the drum
    sqlx::query(r#"DELETE FROM drums WHERE id = $1;"#)
        .bind(request.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting drum: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("Drum deleted! ID: {}", &request.id);
    Ok((StatusCode::OK, Json("Drum deleted!")).into_response())
}
