use std::{str::FromStr, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use rust_decimal::Decimal;
use tracing::{error, info};
use uuid::Uuid;

use crate::models::{
    brand::Brand,
    database::AppState,
    printer::{CreatePrinterRequest, Printer, PrinterDetails, UpdatePrinterRequest},
    supplies::{drum::Drum, toner::Toner},
    DeleteRequest,
};

pub async fn count_printers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let printer_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM printers;"#)
            .fetch_one(&state.db)
            .await;

    match printer_count {
        Ok((count,)) => {
            info!("Successfully retrieved printer count: {}", count);
            Ok(Json(count))
        }
        Err(e) => {
            error!("Error retrieving printer count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving printer count."),
            ))
        }
    }
}

pub async fn search_printer(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    type PrinterView = Option<(
        Uuid,            // printer_id
        String,          // printer_name
        String,          // printer_model
        Uuid,            // brand_id
        String,          // brand_name
        Uuid,            // toner_id
        String,          // toner_name
        Option<i32>,     // toner_stock
        Option<Decimal>, // toner_price
        Uuid,            // drum_id
        String,          // drum_name
        Option<i32>,     // drum_stock
        Option<Decimal>, // drum_price
    )>;

    let printer: Result<PrinterView, sqlx::Error> = sqlx::query_as(
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
    .await;

    match printer {
        Ok(Some(row)) => {
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
            (StatusCode::OK, Json(Some(printer)))
        }
        Ok(None) => {
            error!("No printer found.");
            (StatusCode::NOT_FOUND, Json(None))
        }
        Err(e) => {
            error!("Error retrieving printer: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn show_printers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    type PrintersView = Vec<(
        Uuid,            // printer_id
        String,          // printer_name
        String,          // printer_model
        Uuid,            // brand_id
        String,          // brand_name
        Uuid,            // toner_id
        String,          // toner_name
        Option<i32>,     // toner_stock
        Option<Decimal>, // toner_price
        Uuid,            // drum_id
        String,          // drum_name
        Option<i32>,     // drum_stock
        Option<Decimal>, // drum_price
    )>;

    let printers: Result<PrintersView, sqlx::Error> = sqlx::query_as(
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
    .await;

    match printers {
        Ok(rows) => {
            let printers = rows
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
                .collect::<Vec<PrinterDetails>>();

            info!("Printers listed successfully");
            Ok(Json(printers))
        }
        Err(e) => {
            error!("Error listing printers: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing printers."),
            ))
        }
    }
}

pub async fn create_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreatePrinterRequest>,
) -> impl IntoResponse {
    let new_printer = Printer::new(
        &request.name,
        &request.model,
        Uuid::from_str(&request.brand).unwrap(),
        Uuid::from_str(&request.toner).unwrap(),
        Uuid::from_str(&request.drum).unwrap(),
    );

    // Check duplicate
    match sqlx::query(r#"SELECT id FROM printers WHERE name = $1;"#)
        .bind(&new_printer.name)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            error!("Printer '{}' already exists.", &new_printer.name);
            (StatusCode::CONFLICT, Err(Json("Printer already exists.")))
        }
        Ok(None) => {
            // Name is empty
            if new_printer.name.is_empty() {
                error!("Printer name cannot be empty.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Printer name cannot be empty.")),
                );
            }

            // Name too short
            if new_printer.name.len() < 4 {
                error!("Printer name is too short.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Printer name is too short.")),
                );
            }

            // Name too long
            if new_printer.name.len() > 20 {
                error!("Printer name is too long.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Printer name is too long.")),
                );
            }

            match sqlx::query(
                r#"
                INSERT INTO printers (id, name, model, brand, toner, drum)
                VALUES ($1, $2, $3, $4, $5, $6);
                "#,
            )
            .bind(new_printer.id)
            .bind(new_printer.name)
            .bind(new_printer.model)
            .bind(new_printer.brand)
            .bind(new_printer.toner)
            .bind(new_printer.drum)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Printer created! ID: {}", &new_printer.id);
                    (StatusCode::CREATED, Ok(Json(new_printer.id)))
                }
                Err(e) => {
                    error!("Error creating printer: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error creating printer.")),
                    )
                }
            }
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Err(Json("Error creating printer.")),
        ),
    }
}

pub async fn update_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdatePrinterRequest>,
) -> impl IntoResponse {
    let printer_id = request.id;
    let new_name = request.name;
    let new_model = request.model;
    let new_brand = Uuid::from_str(request.brand.as_str()).unwrap();
    let new_toner = Uuid::from_str(request.toner.as_str()).unwrap();
    let new_drum = Uuid::from_str(request.drum.as_str()).unwrap();

    // ID not found
    match sqlx::query(r#"SELECT id FROM printers WHERE id = $1;"#)
        .bind(printer_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            // Name is empty
            if new_name.is_empty() {
                error!("Printer name cannot be empty.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Printer name cannot be empty.")),
                );
            }

            // Name too short
            if new_name.len() < 4 {
                error!("Printer name is too short.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Printer name is too short.")),
                );
            }

            // Name too long
            if new_name.len() > 20 {
                error!("Printer name is too long.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Printer name is too long.")),
                );
            }

            // Check duplicate
            match sqlx::query(r#"SELECT id FROM printers WHERE name = $1 AND id != $2;"#)
                .bind(&new_name)
                .bind(printer_id)
                .fetch_optional(&state.db)
                .await
            {
                Ok(Some(_)) => {
                    error!("Printer name already exists.");
                    (StatusCode::CONFLICT, Err(Json("Printer already exists.")))
                }
                Ok(None) => {
                    match sqlx::query(
                        r#"UPDATE printers 
                    SET name = $1, model = $2, brand = $3, toner = $4, drum = $5 
                    WHERE id = $6;"#,
                    )
                    .bind(&new_name)
                    .bind(&new_model)
                    .bind(new_brand)
                    .bind(new_toner)
                    .bind(new_drum)
                    .bind(printer_id)
                    .execute(&state.db)
                    .await
                    {
                        Ok(_) => {
                            info!("Printer updated! ID: {}", &printer_id);
                            (StatusCode::OK, Ok(Json(printer_id)))
                        }
                        Err(e) => {
                            error!("Error updating printer: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Err(Json("Error updating printer.")),
                            )
                        }
                    }
                }
                Err(e) => {
                    error!("Error checking for duplicate printer name: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error checking for duplicated printer name.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Printer ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Printer ID not found.")))
        }
        Err(e) => {
            error!("Error fetching printer by ID: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error fetching printer by ID")),
            )
        }
    }
}

pub async fn delete_printer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM printers WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM printers WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Printer deleted! ID: {}", &request.id);
                    (StatusCode::OK, Ok(Json("Printer deleted!")))
                }
                Err(e) => {
                    error!("Error deleting printer: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Ok(Json("Error deleting printer.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Printer ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Printer ID not found")))
        }
        Err(e) => {
            error!("Error deleting printer: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error deleting printer.")),
            )
        }
    }
}
