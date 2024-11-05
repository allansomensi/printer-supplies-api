use crate::{
    errors::api_error::ApiError,
    models::{
        movement::{
            CreateMovementRequest, ItemDetails, Movement, MovementDetails, MovementView,
            PrinterDetails, UpdateMovementRequest,
        },
        DeleteRequest,
    },
    validations::existence::movement_exists,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use infra::database::AppState;
use std::{str::FromStr, sync::Arc};
use tracing::{error, info};
use uuid::Uuid;
use validator::Validate;

/// Retrieves the total count of movements.
///
/// This endpoint counts all movements stored in the database and returns the count as an integer.
/// If no movements are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/movements/count",
    tags = ["Movements"],
    summary = "Get the total count of movements.",
    description = "This endpoint retrieves the total number of movements stored in the database.",
    responses(
        (status = 200, description = "Movement count retrieved successfully", body = i32),
        (status = 500, description = "An error occurred while retrieving the movement count")
    )
)]
pub async fn count_movements(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM movements;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving movement count: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Successfully retrieved movement count: {count}");
    Ok(Json(count))
}

/// Retrieves a specific movement by its ID.
///
/// This endpoint searches for a movement with the specified ID.
/// If the movement is found, it returns the movement details.
#[utoipa::path(
    get,
    path = "/api/v1/movements/{id}",
    tags = ["Movements"],
    summary = "Get a specific movement by ID.",
    description = "This endpoint retrieves a movement's details from the database using its ID. Returns the movement if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the movement to retrieve", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "Movement retrieved successfully", body = MovementDetails),
        (status = 404, description = "No movement found with the specified ID"),
        (status = 500, description = "An error occurred while retrieving the movement")
    )
)]
pub async fn search_movement(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let movement = sqlx::query_as::<_, MovementView>(
        r#"
        SELECT 
            m.id AS movement_id,
            p.id AS printer_id,
            p.name AS printer_name,
            p.model AS printer_model,
            CASE
                WHEN t.id IS NOT NULL THEN t.id
                ELSE d.id
            END AS item_id,
            CASE
                WHEN t.id IS NOT NULL THEN t.name
                ELSE d.name
            END AS item_name,
            m.quantity AS quantity,
            m.created_at AS created_at
        FROM movements m
        JOIN printers p ON m.printer_id = p.id
        LEFT JOIN toners t ON m.item_id = t.id
        LEFT JOIN drums d ON m.item_id = d.id
        WHERE m.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Error retrieving movement with id {id}: {e}");
        ApiError::DatabaseError(e)
    })?;

    match movement {
        Some(row) => {
            let movement = MovementDetails {
                id: row.0,
                printer: PrinterDetails {
                    id: row.1,
                    name: row.2,
                    model: row.3,
                },
                item: ItemDetails {
                    id: row.4,
                    name: row.5,
                },
                quantity: row.6,
                created_at: row.7,
            };

            info!("Movement found: {id}");
            Ok((StatusCode::OK, Json(Some(movement))))
        }
        None => {
            error!("No movement found.");
            Err(ApiError::IdNotFound)
        }
    }
}

/// Retrieves a list of all movements.
///
/// This endpoint fetches all movements stored in the database.
/// If there are no movements, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/movements",
    tags = ["Movements"],
    summary = "List all movements.",
    description = "Fetches all movements stored in the database. If there are no movements, returns an empty array.",
    responses(
        (status = 200, description = "Movements retrieved successfully", body = Vec<MovementDetails>),
        (status = 404, description = "No movements found in the database"),
        (status = 500, description = "An error occurred while retrieving the movements")
    )
)]
pub async fn show_movements(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let movements = sqlx::query_as::<_, MovementView>(
        r#"
        SELECT 
            m.id AS movement_id,
            p.id AS printer_id,
            p.name AS printer_name,
            p.model AS printer_model,
            CASE
                WHEN t.id IS NOT NULL THEN t.id
                ELSE d.id
            END AS item_id,
            CASE
                WHEN t.id IS NOT NULL THEN t.name
                ELSE d.name
            END AS item_name,
            m.quantity AS quantity,
            m.created_at AS created_at
        FROM movements m
        JOIN printers p ON m.printer_id = p.id
        LEFT JOIN toners t ON m.item_id = t.id
        LEFT JOIN drums d ON m.item_id = d.id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Error listing printers: {e}");
        ApiError::DatabaseError(e)
    })?;

    let movements: Vec<MovementDetails> = movements
        .into_iter()
        .map(|row| MovementDetails {
            id: row.0,
            printer: PrinterDetails {
                id: row.1,
                name: row.2,
                model: row.3,
            },
            item: ItemDetails {
                id: row.4,
                name: row.5,
            },
            quantity: row.6,
            created_at: row.7,
        })
        .collect();

    info!("Movements listed successfully");
    Ok(Json(movements))
}

/// Create a new movement.
///
/// This endpoint creates a new movement by providing its details.
#[utoipa::path(
    post,
    path = "/api/v1/movements",
    tags = ["Movements"],
    summary = "Create a new movement.",
    description = "This endpoint creates a new movement in the database with the provided details.",
    request_body = CreateMovementRequest,
    responses(
        (status = 201, description = "Movement created successfully", body = Uuid),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "An error occurred while creating the movement")
    )
)]
pub async fn create_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMovementRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;

    let new_movement = Movement::new(
        Uuid::from_str(&request.printer_id).unwrap(),
        Uuid::from_str(&request.item_id).unwrap(),
        request.quantity,
    );

    // Check if the item exists in toners or drums
    let item_exists: (bool, bool) = sqlx::query_as(
        r#"
        SELECT 
            EXISTS(SELECT 1 FROM toners WHERE id = $1) AS toner_exists,
            EXISTS(SELECT 1 FROM drums WHERE id = $1) AS drum_exists;
        "#,
    )
    .bind(&new_movement.item_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        error!("Database error: {}", e);
        ApiError::DatabaseError(e)
    })?;

    let (toner_exists, drum_exists) = item_exists;

    // Check if the item exists
    if !(toner_exists || drum_exists) {
        error!(
            "Item with ID '{}' not found in toners or drums.",
            &new_movement.item_id
        );
        return Err(ApiError::IdNotFound);
    }

    // Update stock
    let update_stock_query = if toner_exists {
        r#"UPDATE toners SET stock = stock + $1 WHERE id = $2;"#
    } else {
        r#"UPDATE drums SET stock = stock + $1 WHERE id = $2;"#
    };

    sqlx::query(update_stock_query)
        .bind(new_movement.quantity)
        .bind(new_movement.item_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error updating stock: {}", e);
            ApiError::DatabaseError(e)
        })?;

    // Create the movement
    sqlx::query(
        r#"
        INSERT INTO movements (id, printer_id, item_id, quantity, created_at) 
        VALUES ($1, $2, $3, $4, $5);
        "#,
    )
    .bind(new_movement.id)
    .bind(new_movement.printer_id)
    .bind(new_movement.item_id)
    .bind(new_movement.quantity)
    .bind(new_movement.created_at)
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Error creating movement: {}", e);
        ApiError::DatabaseError(e)
    })?;

    info!("Movement created! ID: {}", &new_movement.id);
    Ok((StatusCode::CREATED, Json(new_movement.id)))
}

/// Updates an existing movement.
///
/// This endpoint updates the details of an existing movement.
/// It accepts the movement ID and the new details for the movement.
/// If the movement is successfully updated, it returns the UUID of the updated movement.
#[utoipa::path(
    put,
    path = "/api/v1/movements",
    tags = ["Movements"],
    summary = "Update an existing movement.",
    description = "This endpoint updates the details of an existing movement in the database.",
    request_body = UpdateMovementRequest,
    responses(
        (status = 200, description = "Movement updated successfully", body = Uuid),
        (status = 400, description = "Invalid input"),
        (status = 404, description = "Movement ID not found"),
        (status = 500, description = "An error occurred while updating the movement")
    )
)]
pub async fn update_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateMovementRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    movement_exists(
        state.clone(),
        Uuid::parse_str(request.id.clone().as_str()).unwrap(),
    )
    .await?;

    let movement_id = Uuid::parse_str(request.id.clone().as_str()).unwrap();
    let new_printer_id = request
        .printer_id
        .map(|d| Uuid::from_str(&d).ok())
        .flatten();
    let new_item_id = request.item_id.map(|d| Uuid::from_str(&d).ok()).flatten();
    let new_quantity = request.quantity;

    let mut updated = false;

    // Update printer if provided
    if let Some(printer) = new_printer_id {
        sqlx::query(r#"UPDATE movements SET printer_id = $1 WHERE id = $2;"#)
            .bind(printer)
            .bind(&movement_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating movement printer: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Check if the item is a toner
    let toner_exists =
        sqlx::query_scalar::<_, bool>(r#"SELECT EXISTS(SELECT 1 FROM toners WHERE id = $1);"#)
            .bind(&new_item_id)
            .fetch_one(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating printer name: {e}");
                ApiError::DatabaseError(e)
            })?;

    if toner_exists {
        sqlx::query(r#"UPDATE movements SET item_id = $1 WHERE id = $2;"#)
            .bind(&new_item_id)
            .bind(&movement_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating movement toner: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    } else {
        // If a toner does not exist, check if a drum exists and, if possible, update it.
        let drum_exists =
            sqlx::query_scalar::<_, bool>(r#"SELECT EXISTS(SELECT 1 FROM drums WHERE id = $1);"#)
                .bind(&new_item_id)
                .fetch_one(&state.db)
                .await
                .map_err(|e| {
                    error!("Error updating printer name: {e}");
                    ApiError::DatabaseError(e)
                })?;

        if drum_exists {
            sqlx::query(r#"UPDATE movements SET item_id = $1 WHERE id = $2;"#)
                .bind(&new_item_id)
                .bind(&movement_id)
                .execute(&state.db)
                .await
                .map_err(|e| {
                    error!("Error updating movement drum: {e}");
                    ApiError::DatabaseError(e)
                })?;
            updated = true;
        }
    }

    // Update quantity if provided
    if let Some(quantity) = new_quantity {
        sqlx::query(r#"UPDATE movements SET quantity = $1 WHERE id = $2;"#)
            .bind(&quantity)
            .bind(&movement_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating movement quantity: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    if !updated {
        error!(
            "No updates were made for the provided movement ID: {}",
            &movement_id
        );
        return Err(ApiError::NotModified);
    }

    info!("Movement updated! ID: {}", &movement_id);
    Ok(Json(movement_id))
}

/// Deletes an existing movement.
///
/// This endpoint allows users to delete a specific movement by its ID.
/// It checks if the movement exists before attempting to delete it.
/// If the movement is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
    path = "/api/v1/movements",
    tags = ["Movements"],
    summary = "Delete an existing movement.",
    description = "This endpoint deletes a specific movement from the database using its ID.",
    request_body = DeleteRequest,
    responses(
        (status = 200, description = "Movement deleted successfully", body = String),
        (status = 404, description = "Movement ID not found"),
        (status = 500, description = "An error occurred while deleting the movement")
    )
)]
pub async fn delete_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    movement_exists(state.clone(), request.id.clone()).await?;

    // Delete the movement
    sqlx::query(r#"DELETE FROM movements WHERE id = $1;"#)
        .bind(request.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting movement: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("Movement deleted! ID: {}", &request.id);
    Ok(Json("Movement deleted!"))
}
