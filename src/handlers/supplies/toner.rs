use crate::{
    database::AppState,
    errors::ApiError,
    models::{
        supplies::toner::{CreateTonerRequest, Toner, UpdateTonerRequest},
        DeleteRequest,
    },
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

/// Retrieves the total count of toners.
///
/// This endpoint counts all toners stored in the database and returns the count as an integer.
/// If no toners are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/supplies/toners/count",
    tags = ["Toners"],
    summary = "Get the total count of toners.",
    description = "This endpoint retrieves the total number of toners stored in the database.",
    responses(
        (status = 200, description = "Toner count retrieved successfully", body = i32),
        (status = 500, description = "An error occurred while retrieving the toner count")
    )
)]
pub async fn count_toners(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM toners;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving toner count: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Successfully retrieved toner count: {count}");
    Ok(Json(count))
}

/// Retrieves a specific toner by its ID.
///
/// This endpoint searches for a toner with the specified ID.
/// If the toner is found, it returns the toner details.
#[utoipa::path(
    get,
    path = "/api/v1/supplies/toners/{id}",
    tags = ["Toners"],
    summary = "Get a specific toner by ID.",
    description = "This endpoint retrieves a toner's details from the database using its ID. Returns the toner if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the toner to retrieve", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "Toner retrieved successfully", body = Toner),
        (status = 404, description = "No toner found with the specified ID"),
        (status = 500, description = "An error occurred while retrieving the toner")
    )
)]
pub async fn search_toner(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let toner = sqlx::query_as::<_, Toner>(r#"SELECT * FROM toners WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving toner with id {id}: {e}");
            ApiError::DatabaseError(e)
        })?;

    match toner {
        Some(toner) => {
            info!("Toner found: {id}");
            Ok(Json(toner))
        }
        None => {
            error!("No toner found with id: {id}");
            Err(ApiError::IdNotFound)
        }
    }
}

/// Retrieves a list of all toners.
///
/// This endpoint fetches all toners stored in the database.
/// If there are no toners, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/supplies/toners",
    tags = ["Toners"],
    summary = "List all toners.",
    description = "Fetches all toners stored in the database. If there are no toners, returns an empty array.",
    responses(
        (status = 200, description = "Toners retrieved successfully", body = Vec<Toner>),
        (status = 404, description = "No toners found in the database"),
        (status = 500, description = "An error occurred while retrieving the toners")
    )
)]
pub async fn show_toners(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let toners = sqlx::query_as::<_, Toner>(r#"SELECT * FROM toners;"#)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Error listing toners: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Toners listed successfully");
    Ok(Json(toners))
}

/// Create a new toner.
///
/// This endpoint creates a new toner by providing its details.
/// Validates the toner's name for length and emptiness, checks for duplicates,
/// and inserts the new toner into the database if all validations pass.
#[utoipa::path(
    post,
    path = "/api/v1/supplies/toners",
    tags = ["Toners"],
    summary = "Create a new toner.",
    description = "This endpoint creates a new toner in the database with the provided details.",
    request_body = CreateTonerRequest,
    responses(
        (status = 201, description = "Toner created successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 409, description = "Conflict: Toner with the same name already exists"),
        (status = 500, description = "An error occurred while creating the toner")
    )
)]
pub async fn create_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    match request.validate() {
        Ok(_) => (),
        Err(e) => {
            error!("Validation error: {:?}", e.0);
            return Err(ApiError::ValidationError(e));
        }
    };

    let new_toner = Toner::new(&request.name, request.stock, request.price);

    // Check for duplicate
    let exists = sqlx::query(r#"SELECT id FROM toners WHERE name = $1;"#)
        .bind(&new_toner.name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking for existing toner: {e}");
            ApiError::DatabaseError(e)
        })?
        .is_some();

    if exists {
        error!("Toner '{}' already exists.", &new_toner.name);
        return Err(ApiError::AlreadyExists);
    }

    sqlx::query(r#"INSERT INTO toners (id, name, stock, price) VALUES ($1, $2, $3, $4);"#)
        .bind(new_toner.id)
        .bind(&new_toner.name)
        .bind(new_toner.stock)
        .bind(new_toner.price)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error creating toner: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Toner created! ID: {}", &new_toner.id);
    Ok((StatusCode::CREATED, Json(new_toner.id)))
}

/// Updates an existing toner.
///
/// This endpoint updates the details of an existing toner.
/// It accepts the toner ID and the new details for the toner, including its name, stock, and price.
/// The endpoint validates the new name to ensure it is not empty,
/// does not conflict with an existing toner's name, and meets length requirements.
/// If the toner is successfully updated, it returns the UUID of the updated toner.
#[utoipa::path(
    put,
    path = "/api/v1/supplies/toners",
    tags = ["Toners"],
    summary = "Update an existing toner.",
    description = "This endpoint updates the details of an existing toner in the database.",
    request_body = UpdateTonerRequest,
    responses(
        (status = 200, description = "Toner updated successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 404, description = "Toner ID not found"),
        (status = 409, description = "Conflict: Toner with the same name already exists"),
        (status = 500, description = "An error occurred while updating the toner")
    )
)]
pub async fn update_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateTonerRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let toner_id = request.id;
    let new_name = request.name.clone();
    let new_stock = request.stock;
    let new_price = request.price;

    // ID not found
    let toner_exists = sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(toner_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error fetching toner by ID: {e}");
            ApiError::Unknown
        })?
        .is_some();

    if !toner_exists {
        error!("Toner ID not found.");
        return Err(ApiError::IdNotFound);
    }

    // Validate and update name if provided
    if let Some(name) = new_name {
        if name.is_empty() {
            error!("Toner name cannot be empty.");
            return Err(ApiError::EmptyName);
        }
        if name.len() < 4 {
            error!("Toner name is too short.");
            return Err(ApiError::NameTooShort);
        }
        if name.len() > 20 {
            error!("Toner name is too long.");
            return Err(ApiError::NameTooLong);
        }

        // Check for duplicate
        let name_exists = sqlx::query(r#"SELECT id FROM toners WHERE name = $1 AND id != $2;"#)
            .bind(&name)
            .bind(toner_id)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                error!("Error checking for duplicate toner name: {e}");
                ApiError::Unknown
            })?
            .is_some();

        if name_exists {
            error!("Toner name already exists.");
            return Err(ApiError::AlreadyExists);
        }

        // Update toner name
        sqlx::query(r#"UPDATE toners SET name = $1 WHERE id = $2;"#)
            .bind(&name)
            .bind(toner_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating toner name: {e}");
                ApiError::DatabaseError(e)
            })?;
    }

    // Update stock if provided
    if let Some(stock) = new_stock {
        sqlx::query(r#"UPDATE toners SET stock = $1 WHERE id = $2;"#)
            .bind(stock)
            .bind(toner_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating toner stock: {e}");
                ApiError::DatabaseError(e)
            })?;
    }

    // Update price if provided
    if let Some(price) = new_price {
        sqlx::query(r#"UPDATE toners SET price = $1 WHERE id = $2;"#)
            .bind(price)
            .bind(toner_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating toner price: {e}");
                ApiError::DatabaseError(e)
            })?;
    }

    info!("Toner updated! ID: {}", &toner_id);
    Ok((StatusCode::OK, Json(toner_id)).into_response())
}

/// Deletes an existing toner.
///
/// This endpoint allows users to delete a specific toner by its ID.
/// It checks if the toner exists before attempting to delete it.
/// If the toner is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
    path = "/api/v1/supplies/toners",
    tags = ["Toners"],
    summary = "Delete an existing toner.",
    description = "This endpoint deletes a specific toner from the database using its ID.",
    request_body = DeleteRequest,
    responses(
        (status = 200, description = "Toner deleted successfully", body = String),
        (status = 404, description = "Toner ID not found"),
        (status = 500, description = "An error occurred while deleting the toner")
    )
)]
pub async fn delete_toner(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Check if the toner exists
    let toner_exists = sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error checking toner ID: {}", e);
            ApiError::Unknown
        })?
        .is_some();

    if !toner_exists {
        error!("Toner ID not found.");
        return Err(ApiError::IdNotFound);
    }

    // Delete the toner
    sqlx::query(r#"DELETE FROM toners WHERE id = $1;"#)
        .bind(request.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting toner: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("Toner deleted! ID: {}", &request.id);
    Ok((StatusCode::OK, Json("Toner deleted!")).into_response())
}
