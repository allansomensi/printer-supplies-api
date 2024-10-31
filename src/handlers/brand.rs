use crate::{
    database::AppState,
    errors::api_error::ApiError,
    models::{
        brand::{Brand, CreateBrandRequest, UpdateBrandRequest},
        DeleteRequest,
    },
    validations::{existence::brand_exists, uniqueness::is_brand_unique},
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

/// Retrieves the total count of brands.
///
/// This endpoint counts all brands stored in the database and returns the count as an integer.
/// If no brands are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/brands/count",
    tags = ["Brands"],
    summary = "Get the total count of brands.",
    description = "This endpoint retrieves the total number of brands stored in the database.",
    responses(
        (status = 200, description = "Brand count retrieved successfully", body = i32),
        (status = 500, description = "An error occurred while retrieving the brand count")
    )
)]
pub async fn count_brands(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM brands;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving brand count: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Successfully retrieved brand count: {count}");
    Ok(Json(count))
}

/// Retrieves a specific brand by its ID.
///
/// This endpoint searches for a brand with the specified ID.
/// If the brand is found, it returns the brand details.
#[utoipa::path(
    get,
    path = "/api/v1/brands/{id}",
    tags = ["Brands"],
    summary = "Get a specific brand by ID.",
    description = "This endpoint retrieves a brand's details from the database using its ID. Returns the brand if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the brand to retrieve", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "Brand retrieved successfully", body = Brand),
        (status = 404, description = "No brand found with the specified ID"),
        (status = 500, description = "An error occurred while retrieving the brand")
    )
)]
pub async fn search_brand(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let brand = sqlx::query_as::<_, Brand>(r#"SELECT * FROM brands WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving brand with id {id}: {e}");
            ApiError::DatabaseError(e)
        })?;

    match brand {
        Some(brand) => {
            info!("Brand found: {id}");
            Ok(Json(brand))
        }
        None => {
            error!("No brand found with id: {id}");
            Err(ApiError::IdNotFound)
        }
    }
}

/// Retrieves a list of all brands.
///
/// This endpoint fetches all brands stored in the database.
/// If there are no brands, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/brands",
    tags = ["Brands"],
    summary = "List all brands.",
    description = "Fetches all brands stored in the database. If there are no brands, returns an empty array.",
    responses(
        (status = 200, description = "Brands retrieved successfully", body = Vec<Brand>),
        (status = 404, description = "No brands found in the database"),
        (status = 500, description = "An error occurred while retrieving the brands")
    )
)]
pub async fn show_brands(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let brands = sqlx::query_as::<_, Brand>(r#"SELECT * FROM brands;"#)
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Error listing brands: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Brands listed successfully");
    Ok(Json(brands))
}

/// Create a new brand.
///
/// This endpoint creates a new brand by providing its details.
/// Validates the brand's name for length and emptiness, checks for duplicates,
/// and inserts the new brand into the database if all validations pass.
#[utoipa::path(
    post,
    path = "/api/v1/brands",
    tags = ["Brands"],
    summary = "Create a new brand.",
    description = "This endpoint creates a new brand in the database with the provided details.",
    request_body = CreateBrandRequest,
    responses(
        (status = 201, description = "Brand created successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 409, description = "Conflict: Brand with the same name already exists"),
        (status = 500, description = "An error occurred while creating the brand")
    )
)]
pub async fn create_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateBrandRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    is_brand_unique(state.clone(), request.name.clone()).await?;

    let new_brand = Brand::new(&request.name);

    // Creates the brand.
    sqlx::query(r#"INSERT INTO brands (id, name) VALUES ($1, $2)"#)
        .bind(new_brand.id)
        .bind(&new_brand.name)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error creating brand: {e}");
            ApiError::DatabaseError(e)
        })?;
    info!("Brand created! ID: {}", &new_brand.id);
    Ok((StatusCode::CREATED, Json(new_brand.id)))
}

/// Updates an existing brand.
///
/// This endpoint updates the details of an existing brand.
/// It accepts the brand ID and the new details for the brand.
/// The endpoint validates the new name to ensure it is not empty,
/// does not conflict with an existing brand's name, and meets length requirements.
/// If the brand is successfully updated, it returns the UUID of the updated brand.
#[utoipa::path(
    put,
    path = "/api/v1/brands",
    tags = ["Brands"],
    summary = "Update an existing brand.",
    description = "This endpoint updates the details of an existing brand in the database.",
    request_body = UpdateBrandRequest,
    responses(
        (status = 200, description = "Brand updated successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 404, description = "Brand ID not found"),
        (status = 409, description = "Conflict: Brand with the same name already exists"),
        (status = 500, description = "An error occurred while updating the brand")
    )
)]
pub async fn update_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateBrandRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    brand_exists(state.clone(), request.id.clone()).await?;

    let brand_id = request.id;
    let new_name = request.name;

    // Update the brand
    sqlx::query(r#"UPDATE brands SET name = $1 WHERE id = $2;"#)
        .bind(&new_name)
        .bind(brand_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error updating brand name: {e}");
            ApiError::DatabaseError(e)
        })?;
    info!("Brand updated! ID: {}", &brand_id);
    Ok((StatusCode::OK, Json(brand_id)).into_response())
}

/// Deletes an existing brand.
///
/// This endpoint allows users to delete a specific brand by its ID.
/// It checks if the brand exists before attempting to delete it.
/// If the brand is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
    path = "/api/v1/brands",
    tags = ["Brands"],
    summary = "Delete an existing brand.",
    description = "This endpoint deletes a specific brand from the database using its ID.",
    request_body = DeleteRequest,
    responses(
        (status = 200, description = "Brand deleted successfully", body = String),
        (status = 404, description = "Brand ID not found"),
        (status = 500, description = "An error occurred while deleting the brand")
    )
)]
pub async fn delete_brand(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    brand_exists(state.clone(), request.id.clone()).await?;

    // Delete the brand
    sqlx::query(r#"DELETE FROM brands WHERE id = $1;"#)
        .bind(request.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting brand: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("Brand deleted! ID: {}", &request.id);
    Ok((StatusCode::OK, Json("Brand deleted!")).into_response())
}
