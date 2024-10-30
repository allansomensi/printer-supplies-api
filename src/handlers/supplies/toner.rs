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
        supplies::toner::{CreateTonerRequest, Toner, UpdateTonerRequest},
        DeleteRequest,
    },
};

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
pub async fn count_toners(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let toner_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM toners;"#)
            .fetch_one(&state.db)
            .await;
    match toner_count {
        Ok((count,)) => {
            info!("Successfully retrieved toner count: {}", count);
            Ok(Json(count))
        }
        Err(e) => {
            error!("Error retrieving toner count: {e}");
            Err(ApiError::DatabaseError(e))
        }
    }
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
) -> impl IntoResponse {
    match sqlx::query_as::<_, Toner>(r#"SELECT * FROM toners WHERE id = $1;"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(toner)) => {
            info!("Toner found: {id}");
            (StatusCode::OK, Json(Some(toner))).into_response()
        }
        Ok(None) => {
            error!("No toner found.");
            (ApiError::IdNotFound).into_response()
        }
        Err(e) => {
            error!("Error retrieving toner: {e}");
            (ApiError::DatabaseError(e)).into_response()
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
pub async fn show_toners(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let toners: Result<Vec<Toner>, sqlx::Error> = sqlx::query_as(r#"SELECT * FROM toners;"#)
        .fetch_all(&state.db)
        .await;
    match toners {
        Ok(toners) => {
            info!("Toners listed successfully");
            Ok(Json(toners))
        }
        Err(e) => {
            error!("Error listing toners: {e}");
            Err(ApiError::DatabaseError(e))
        }
    }
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
) -> impl IntoResponse {
    // Validations

    // Name is empty
    if request.name.is_empty() {
        error!("Toner name cannot be empty.");
        return ApiError::EmptyName.into_response();
    }
    // Name too short
    if request.name.len() < 4 {
        error!("Toner name is too short.");
        return ApiError::NameTooShort.into_response();
    }
    // Name too long
    if request.name.len() > 20 {
        error!("Toner name is too long.");
        return ApiError::NameTooLong.into_response();
    }

    let new_toner = Toner::new(&request.name, request.stock, request.price);

    // Check for duplicate toner name
    match sqlx::query(r#"SELECT id FROM toners WHERE name = $1;"#)
        .bind(&new_toner.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Toner '{}' already exists.", &new_toner.name);
            ApiError::AlreadyExists.into_response()
        }
        Ok(None) => {
            match sqlx::query(
                r#"INSERT INTO toners (id, name, stock, price) VALUES ($1, $2, $3, $4);"#,
            )
            .bind(new_toner.id)
            .bind(&new_toner.name)
            .bind(new_toner.stock)
            .bind(new_toner.price)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Toner created! ID: {}", &new_toner.id);
                    (StatusCode::CREATED, Json(new_toner.id)).into_response()
                }
                Err(e) => {
                    error!("Error creating toner: {}", e);
                    ApiError::DatabaseError(e).into_response()
                }
            }
        }
        Err(e) => ApiError::DatabaseError(e).into_response(),
    }
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
) -> impl IntoResponse {
    let toner_id = request.id;
    let new_name = request.name.clone();
    let new_stock = request.stock;
    let new_price = request.price;

    // ID not found
    match sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(toner_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            if let Some(name) = new_name {
                if name.is_empty() {
                    error!("Toner name cannot be empty.");
                    return ApiError::EmptyName.into_response();
                }

                if name.len() < 4 {
                    error!("Toner name is too short.");
                    return ApiError::NameTooShort.into_response();
                }

                if name.len() > 20 {
                    error!("Toner name is too long.");
                    return ApiError::NameTooLong.into_response();
                }

                // Check duplicate
                match sqlx::query(r#"SELECT id FROM toners WHERE name = $1 AND id != $2;"#)
                    .bind(&name)
                    .bind(toner_id)
                    .fetch_optional(&state.db)
                    .await
                {
                    Ok(Some(_)) => {
                        error!("Toner name already exists.");
                        return ApiError::AlreadyExists.into_response();
                    }
                    Ok(None) => {
                        if let Err(e) = sqlx::query(r#"UPDATE toners SET name = $1 WHERE id = $2;"#)
                            .bind(&name)
                            .bind(toner_id)
                            .execute(&state.db)
                            .await
                        {
                            error!("Error updating toner name: {e}");
                            return ApiError::DatabaseError(e).into_response();
                        }
                    }
                    Err(e) => {
                        error!("Error checking for duplicate toner name: {e}");
                        return ApiError::Unknown.into_response();
                    }
                }
            }

            if let Some(stock) = new_stock {
                if let Err(e) = sqlx::query(r#"UPDATE toners SET stock = $1 WHERE id = $2;"#)
                    .bind(stock)
                    .bind(toner_id)
                    .execute(&state.db)
                    .await
                {
                    error!("Error updating toner stock: {}", e);
                    return ApiError::DatabaseError(e).into_response();
                }
            }

            if let Some(price) = new_price {
                if let Err(e) = sqlx::query(r#"UPDATE toners SET price = $1 WHERE id = $2;"#)
                    .bind(price)
                    .bind(toner_id)
                    .execute(&state.db)
                    .await
                {
                    error!("Error updating toner price: {e}");
                    return ApiError::DatabaseError(e).into_response();
                }
            }

            info!("Toner updated! ID: {}", &toner_id);
            (StatusCode::OK, Json(toner_id)).into_response()
        }
        Ok(None) => {
            error!("Toner ID not found.");
            ApiError::IdNotFound.into_response()
        }
        Err(e) => {
            error!("Error fetching toner by ID: {e}");
            ApiError::Unknown.into_response()
        }
    }
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
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM toners WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM toners WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Toner deleted! ID: {}", &request.id);
                    (StatusCode::OK, Json("Toner deleted!")).into_response()
                }
                Err(e) => {
                    error!("Error deleting toner: {}", e);
                    ApiError::DatabaseError(e).into_response()
                }
            }
        }
        Ok(None) => {
            error!("Toner ID not found.");
            ApiError::IdNotFound.into_response()
        }
        Err(e) => {
            error!("Error deleting toner: {}", e);
            ApiError::Unknown.into_response()
        }
    }
}
