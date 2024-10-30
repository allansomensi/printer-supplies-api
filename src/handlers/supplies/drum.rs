use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::{
        database::AppState,
        supplies::drum::{CreateDrumRequest, Drum, UpdateDrumRequest},
        DeleteRequest,
    },
};

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
pub async fn count_drums(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let drum_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM drums;"#)
            .fetch_one(&state.db)
            .await;

    match drum_count {
        Ok((count,)) => {
            info!("Successfully retrieved drum count: {}", count);
            Ok(Json(count))
        }
        Err(e) => {
            error!("Error retrieving drum count: {e}");
            Err(ApiError::DatabaseError(e))
        }
    }
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
    match sqlx::query_as::<_, Drum>(r#"SELECT * FROM drums WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(drum)) => {
            info!("Drum found: {id}");
            (StatusCode::OK, Json(Some(drum))).into_response()
        }
        Ok(None) => {
            error!("No drum found.");
            (ApiError::IdNotFound).into_response()
        }
        Err(e) => {
            error!("Error retrieving drum: {e}");
            (ApiError::DatabaseError(e)).into_response()
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
pub async fn show_drums(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let drums: Result<Vec<Drum>, sqlx::Error> = sqlx::query_as(r#"SELECT * FROM drums;"#)
        .fetch_all(&state.db)
        .await;
    match drums {
        Ok(drums) => {
            info!("Drums listed successfully");
            Ok(Json(drums))
        }
        Err(e) => {
            error!("Error listing drums: {e}");
            Err(ApiError::DatabaseError(e))
        }
    }
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
) -> impl IntoResponse {
    // Validations

    // Name is empty
    if request.name.is_empty() {
        error!("Drum name cannot be empty.");
        return ApiError::EmptyName.into_response();
    }
    // Name too short
    if request.name.len() < 4 {
        error!("Drum name is too short.");
        return ApiError::NameTooShort.into_response();
    }
    // Name too long
    if request.name.len() > 20 {
        error!("Drum name is too long.");
        return ApiError::NameTooLong.into_response();
    }

    let new_drum = Drum::new(&request.name, request.stock, request.price);

    // Check for duplicate drum name
    match sqlx::query(r#"SELECT id FROM drums WHERE name = $1;"#)
        .bind(&new_drum.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Drum '{}' already exists.", &new_drum.name);
            ApiError::AlreadyExists.into_response()
        }
        Ok(None) => {
            match sqlx::query(
                r#"INSERT INTO drums (id, name, stock, price) VALUES ($1, $2, $3, $4);"#,
            )
            .bind(new_drum.id)
            .bind(&new_drum.name)
            .bind(new_drum.stock)
            .bind(new_drum.price)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Drum created! ID: {}", &new_drum.id);
                    (StatusCode::CREATED, Json(new_drum.id)).into_response()
                }
                Err(e) => {
                    error!("Error creating drum: {}", e);
                    ApiError::DatabaseError(e).into_response()
                }
            }
        }
        Err(e) => ApiError::DatabaseError(e).into_response(),
    }
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
) -> impl IntoResponse {
    let drum_id = request.id;
    let new_name = request.name.clone();
    let new_stock = request.stock;
    let new_price = request.price;

    // ID not found
    match sqlx::query(r#"SELECT id FROM drums WHERE id = $1;"#)
        .bind(drum_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            if let Some(name) = new_name {
                if name.is_empty() {
                    error!("Drum name cannot be empty.");
                    return ApiError::EmptyName.into_response();
                }

                if name.len() < 4 {
                    error!("Drum name is too short.");
                    return ApiError::NameTooShort.into_response();
                }

                if name.len() > 20 {
                    error!("Drum name is too long.");
                    return ApiError::NameTooLong.into_response();
                }

                // Check duplicate
                match sqlx::query(r#"SELECT id FROM drums WHERE name = $1 AND id != $2;"#)
                    .bind(&name)
                    .bind(drum_id)
                    .fetch_optional(&state.db)
                    .await
                {
                    Ok(Some(_)) => {
                        error!("Drum name already exists.");
                        return ApiError::AlreadyExists.into_response();
                    }
                    Ok(None) => {
                        if let Err(e) = sqlx::query(r#"UPDATE drums SET name = $1 WHERE id = $2;"#)
                            .bind(&name)
                            .bind(drum_id)
                            .execute(&state.db)
                            .await
                        {
                            error!("Error updating drum name: {e}");
                            return ApiError::DatabaseError(e).into_response();
                        }
                    }
                    Err(e) => {
                        error!("Error checking for duplicate drum name: {e}");
                        return ApiError::Unknown.into_response();
                    }
                }
            }

            if let Some(stock) = new_stock {
                if let Err(e) = sqlx::query(r#"UPDATE drums SET stock = $1 WHERE id = $2;"#)
                    .bind(stock)
                    .bind(drum_id)
                    .execute(&state.db)
                    .await
                {
                    error!("Error updating drum stock: {e}");
                    return ApiError::DatabaseError(e).into_response();
                }
            }

            if let Some(price) = new_price {
                if let Err(e) = sqlx::query(r#"UPDATE drums SET price = $1 WHERE id = $2;"#)
                    .bind(price)
                    .bind(drum_id)
                    .execute(&state.db)
                    .await
                {
                    error!("Error updating drum price: {e}");
                    return ApiError::DatabaseError(e).into_response();
                }
            }

            info!("Drum updated! ID: {}", &drum_id);
            (StatusCode::OK, Json(drum_id)).into_response()
        }
        Ok(None) => {
            error!("Drum ID not found.");
            ApiError::IdNotFound.into_response()
        }
        Err(e) => {
            error!("Error fetching drum by ID: {e}");
            ApiError::Unknown.into_response()
        }
    }
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
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM drums WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM drums WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Drum deleted! ID: {}", &request.id);
                    (StatusCode::OK, Json("Drum deleted!")).into_response()
                }
                Err(e) => {
                    error!("Error deleting drum: {}", e);
                    ApiError::DatabaseError(e).into_response()
                }
            }
        }
        Ok(None) => {
            error!("Drum ID not found.");
            ApiError::IdNotFound.into_response()
        }
        Err(e) => {
            error!("Error deleting drum: {}", e);
            ApiError::Unknown.into_response()
        }
    }
}
