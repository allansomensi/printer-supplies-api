use crate::{
    errors::api_error::ApiError,
    models::{
        brand::Brand,
        printer::{
            CreatePrinterRequest, Printer, PrinterDetails, PrinterView, UpdatePrinterRequest,
        },
        supplies::{drum::Drum, toner::Toner},
        DeleteRequest,
    },
    validations::{existence::printer_exists, uniqueness::is_printer_unique},
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

/// Retrieves the total count of printers.
///
/// This endpoint counts all printers stored in the database and returns the count as an integer.
/// If no printers are found, 0 is returned.
#[utoipa::path(
    get,
    path = "/api/v1/printers/count",
    tags = ["Printers"],
    summary = "Get the total count of printers.",
    description = "This endpoint retrieves the total number of printers stored in the database.",
    responses(
        (status = 200, description = "Printer count retrieved successfully", body = i32),
        (status = 500, description = "An error occurred while retrieving the printer count")
    )
)]
pub async fn count_printers(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let count = sqlx::query_scalar::<_, i64>(r#"SELECT COUNT(*) FROM printers;"#)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Error retrieving printer count: {e}");
            ApiError::DatabaseError(e)
        })?;

    info!("Successfully retrieved printer count: {count}");
    Ok(Json(count))
}

/// Retrieves a specific printer by its ID.
///
/// This endpoint searches for a printer with the specified ID.
/// If the printer is found, it returns the printer details.
#[utoipa::path(
    get,
    path = "/api/v1/printers/{id}",
    tags = ["Printers"],
    summary = "Get a specific printer by ID.",
    description = "This endpoint retrieves a printer's details from the database using its ID. Returns the printer if found, or a 404 status if not found.",
    params(
        ("id", description = "The unique identifier of the printer to retrieve", example = "550e8400-e29b-41d4-a716-446655440000")
    ),
    responses(
        (status = 200, description = "Printer retrieved successfully", body = PrinterDetails),
        (status = 404, description = "No printer found with the specified ID"),
        (status = 500, description = "An error occurred while retrieving the printer")
    )
)]
pub async fn search_printer(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let printer = sqlx::query_as::<_, PrinterView>(
        r#"
        SELECT 
            p.id AS printer_id, 
            p.name AS printer_name, 
            p.model AS printer_model,
            p.brand AS brand_id, 
            b.name AS brand_name,
            p.toner AS toner_id, 
            t.name AS toner_name, 
            t.stock AS toner_stock,
            t.price AS toner_price,
            p.drum AS drum_id,
            d.name AS drum_name, 
            d.stock AS drum_stock,
            d.price AS drum_price
        FROM printers p
        JOIN toners t ON p.toner = t.id
        JOIN drums d ON p.drum = d.id
        JOIN brands b ON p.brand = b.id
        WHERE p.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        error!("Error retrieving printer with id {id}: {e}");
        ApiError::DatabaseError(e)
    })?;

    match printer {
        Some(row) => {
            let printer = PrinterDetails {
                id: row.0,
                name: row.1,
                model: row.2,
                brand: Brand {
                    id: row.3,
                    name: row.4,
                },
                toner: Toner {
                    id: row.5,
                    name: row.6,
                    stock: row.7,
                    price: row.8,
                },
                drum: Drum {
                    id: row.9,
                    name: row.10,
                    stock: row.11,
                    price: row.12,
                },
            };

            info!("Printer found: {id}");
            Ok((StatusCode::OK, Json(Some(printer))))
        }
        None => {
            error!("No printer found.");
            Err(ApiError::IdNotFound)
        }
    }
}

/// Retrieves a list of all printers.
///
/// This endpoint fetches all printers stored in the database.
/// If there are no printers, returns an empty array.
#[utoipa::path(
    get,
    path = "/api/v1/printers",
    tags = ["Printers"],
    summary = "List all printers.",
    description = "Fetches all printers stored in the database. If there are no printers, returns an empty array.",
    responses(
        (status = 200, description = "Printers retrieved successfully", body = Vec<PrinterDetails>),
        (status = 404, description = "No printers found in the database"),
        (status = 500, description = "An error occurred while retrieving the printers")
    )
)]
pub async fn show_printers(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    let printers = sqlx::query_as::<_, PrinterView>(
        r#"
        SELECT 
            p.id AS printer_id, 
            p.name AS printer_name, 
            p.model AS printer_model,
            p.brand AS brand_id, 
            b.name AS brand_name,
            p.toner AS toner_id, 
            t.name AS toner_name, 
            t.stock AS toner_stock,
            t.price AS toner_price,
            p.drum AS drum_id,
            d.name AS drum_name, 
            d.stock AS drum_stock,
            d.price AS drum_price
        FROM printers p
        JOIN toners t ON p.toner = t.id
        JOIN drums d ON p.drum = d.id
        JOIN brands b ON p.brand = b.id
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Error listing printers: {e}");
        ApiError::DatabaseError(e)
    })?;

    let printers: Vec<PrinterDetails> = printers
        .into_iter()
        .map(|row| PrinterDetails {
            id: row.0,
            name: row.1,
            model: row.2,
            brand: Brand {
                id: row.3,
                name: row.4,
            },
            toner: Toner {
                id: row.5,
                name: row.6,
                stock: row.7,
                price: row.8,
            },
            drum: Drum {
                id: row.9,
                name: row.10,
                stock: row.11,
                price: row.12,
            },
        })
        .collect();

    info!("Printers listed successfully");
    Ok(Json(printers))
}

/// Create a new printer.
///
/// This endpoint creates a new printer by providing its details.
/// Validates the printer's name for length and emptiness, checks for duplicates,
/// and inserts the new printer into the database if all validations pass.
#[utoipa::path(
    post,
    path = "/api/v1/printers",
    tags = ["Printers"],
    summary = "Create a new printer.",
    description = "This endpoint creates a new printer in the database with the provided details.",
    request_body = CreatePrinterRequest,
    responses(
        (status = 201, description = "Printer created successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 409, description = "Conflict: Printer with the same name already exists"),
        (status = 500, description = "An error occurred while creating the printer")
    )
)]
pub async fn create_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePrinterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    is_printer_unique(state.clone(), request.name.clone()).await?;

    let new_printer = Printer::new(
        &request.name,
        &request.model,
        Uuid::from_str(&request.brand).unwrap(),
        Uuid::from_str(&request.toner).unwrap(),
        Uuid::from_str(&request.drum).unwrap(),
    );

    sqlx::query(r#"INSERT INTO printers (id, name, model, brand, toner, drum) VALUES ($1, $2, $3, $4, $5, $6);"#,
    )
    .bind(new_printer.id)
    .bind(new_printer.name)
    .bind(new_printer.model)
    .bind(new_printer.brand)
    .bind(new_printer.toner)
    .bind(new_printer.drum)
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Error creating printer: {e}");
        ApiError::DatabaseError(e)
    })?;

    info!("Printer created! ID: {}", &new_printer.id);
    Ok((StatusCode::CREATED, Json(new_printer.id)))
}

/// Updates an existing printer.
///
/// This endpoint updates the details of an existing printer.
/// It accepts the printer ID and the new details for the printer.
/// The endpoint validates the new name to ensure it is not empty,
/// does not conflict with an existing printer's name, and meets length requirements.
/// If the printer is successfully updated, it returns the UUID of the updated printer.
#[utoipa::path(
    put,
    path = "/api/v1/printers",
    tags = ["Printers"],
    summary = "Update an existing printer.",
    description = "This endpoint updates the details of an existing printer in the database.",
    request_body = UpdatePrinterRequest,
    responses(
        (status = 200, description = "Printer updated successfully", body = Uuid),
        (status = 400, description = "Invalid input, including empty name or name too short/long"),
        (status = 404, description = "Printer ID not found"),
        (status = 304, description = "Printer not modified"),
        (status = 409, description = "Conflict: Printer with the same name already exists"),
        (status = 500, description = "An error occurred while updating the printer")
    )
)]
pub async fn update_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdatePrinterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    request.validate()?;
    printer_exists(state.clone(), request.id.clone()).await?;

    let printer_id = request.id;
    let new_name = request.name;
    let new_model = request.model;
    let new_brand_id = request.brand.map(|b| Uuid::from_str(&b).ok()).flatten();
    let new_toner_id = request.toner.map(|t| Uuid::from_str(&t).ok()).flatten();
    let new_drum_id = request.drum.map(|d| Uuid::from_str(&d).ok()).flatten();

    let mut updated = false;

    // Update name if provided
    if let Some(name) = new_name {
        sqlx::query(r#"UPDATE printers SET name = $1 WHERE id = $2;"#)
            .bind(&name)
            .bind(printer_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating printer name: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update model if provided
    if let Some(model) = new_model {
        sqlx::query(r#"UPDATE printers SET model = $1 WHERE id = $2;"#)
            .bind(&model)
            .bind(printer_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating printer model: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update brand if provided
    if let Some(brand) = new_brand_id {
        sqlx::query(r#"UPDATE printers SET brand = $1 WHERE id = $2;"#)
            .bind(brand)
            .bind(printer_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating printer brand: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update toner if provided
    if let Some(toner) = new_toner_id {
        sqlx::query(r#"UPDATE printers SET toner = $1 WHERE id = $2;"#)
            .bind(toner)
            .bind(printer_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating printer toner: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    // Update drum if provided
    if let Some(drum) = new_drum_id {
        sqlx::query(r#"UPDATE printers SET drum = $1 WHERE id = $2;"#)
            .bind(drum)
            .bind(printer_id)
            .execute(&state.db)
            .await
            .map_err(|e| {
                error!("Error updating printer drum: {e}");
                ApiError::DatabaseError(e)
            })?;
        updated = true;
    }

    if !updated {
        error!(
            "No updates were made for the provided printer ID: {}",
            &printer_id
        );
        return Err(ApiError::NotModified);
    }

    info!("Printer updated! ID: {}", &printer_id);
    Ok(Json(printer_id))
}

/// Deletes an existing printer.
///
/// This endpoint allows users to delete a specific printer by its ID.
/// It checks if the printer exists before attempting to delete it.
/// If the printer is successfully deleted, a confirmation message is returned.
#[utoipa::path(
    delete,
    path = "/api/v1/printers",
    tags = ["Printers"],
    summary = "Delete an existing printer.",
    description = "This endpoint deletes a specific printer from the database using its ID.",
    request_body = DeleteRequest,
    responses(
        (status = 200, description = "Printer deleted successfully", body = String),
        (status = 404, description = "Printer ID not found"),
        (status = 500, description = "An error occurred while deleting the printer")
    )
)]
pub async fn delete_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validations
    printer_exists(state.clone(), request.id.clone()).await?;

    // Delete the printer
    sqlx::query(r#"DELETE FROM printers WHERE id = $1;"#)
        .bind(request.id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!("Error deleting printer: {}", e);
            ApiError::DatabaseError(e)
        })?;

    info!("Printer deleted! ID: {}", &request.id);
    Ok(Json("Printer deleted!"))
}
