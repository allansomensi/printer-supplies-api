use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::{error, info};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::models::{
    database::AppState,
    movement::{
        CreateMovementRequest, ItemDetails, Movement, MovementDetails, PrinterDetails,
        UpdateMovementRequest,
    },
    DeleteRequest,
};

pub async fn count_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Ok((StatusCode::OK, Json(count)))
        }
        Err(e) => {
            error!("Error retrieving movement count: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving movement count."),
            ))
        }
    }
}

pub async fn search_movement(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Movement>(r#"SELECT * FROM movements WHERE id = $1"#)
        .bind(id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(movement)) => {
            info!("Movement found: {id}");
            (StatusCode::OK, Json(Some(movement)))
        }
        Ok(None) => {
            error!("No movement found.");
            (StatusCode::NOT_FOUND, Json(None))
        }
        Err(e) => {
            error!("Error retrieving movement: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

pub async fn show_movements(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    type MovementView = Vec<(
        Uuid,
        Uuid,
        String,
        String,
        Uuid,
        String,
        i32,
        i32,
        DateTime<Utc>,
    )>;

    let movements: Result<MovementView, sqlx::Error> = sqlx::query_as(
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
            CASE
                WHEN t.id IS NOT NULL THEN t.stock
                ELSE d.stock
            END AS item_stock,
            m.quantity AS quantity,
            m.created_at AS created_at
        FROM movements m
        JOIN printers p ON m.printer_id = p.id
        LEFT JOIN toners t ON m.item_id = t.id
        LEFT JOIN drums d ON m.item_id = d.id
        "#,
    )
    .fetch_all(&state.db)
    .await;

    match movements {
        Ok(rows) => {
            let movements = rows
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
                        stock: row.6,
                    },
                    quantity: row.7,
                    created_at: row.8,
                })
                .collect::<Vec<MovementDetails>>();

            info!("Movements listed successfully");
            Ok(Json(movements))
        }
        Err(e) => {
            error!("Error listing movements: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error listing movements."),
            ))
        }
    }
}
pub async fn create_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateMovementRequest>,
) -> impl IntoResponse {
    let new_movement = Movement::new(request.printer_id, request.item_id, request.quantity);

    // Check if the item exists in toners or drums
    let toner_exists =
        sqlx::query_scalar::<_, bool>(r#"SELECT EXISTS(SELECT 1 FROM toners WHERE id = $1);"#)
            .bind(request.item_id)
            .fetch_one(&state.db)
            .await;

    let drum_exists =
        sqlx::query_scalar::<_, bool>(r#"SELECT EXISTS(SELECT 1 FROM drums WHERE id = $1);"#)
            .bind(request.item_id)
            .fetch_one(&state.db)
            .await;

    match (&toner_exists, &drum_exists) {
        (Ok(true), _) | (_, Ok(true)) => {
            // Check that quantity is not zero
            if new_movement.quantity == 0 {
                error!("Quantity cannot be zero.");
                return (
                    StatusCode::BAD_REQUEST,
                    Err(Json("Quantity must be non-zero.")),
                );
            }

            // Update stock
            let update_stock = if toner_exists.unwrap() {
                r#"UPDATE toners SET stock = stock + $1 WHERE id = $2;"#
            } else {
                r#"UPDATE drums SET stock = stock + $1 WHERE id = $2;"#
            };

            match sqlx::query(update_stock)
                .bind(new_movement.quantity)
                .bind(request.item_id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    // Insert the new movement record
                    match sqlx::query(
                        r#"
                        INSERT INTO movements (id, printer_id, item_id, quantity, created_at) 
                        VALUES ($1, $2, $3, $4, $5);"#,
                    )
                    .bind(new_movement.id)
                    .bind(new_movement.printer_id)
                    .bind(new_movement.item_id)
                    .bind(new_movement.quantity)
                    .bind(new_movement.created_at)
                    .execute(&state.db)
                    .await
                    {
                        Ok(_) => {
                            info!("Movement created! ID: {}", &new_movement.id);
                            (StatusCode::CREATED, Ok(Json(new_movement.id)))
                        }
                        Err(e) => {
                            error!("Error creating movement: {}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Err(Json("Error creating movement.")),
                            )
                        }
                    }
                }
                Err(e) => {
                    error!("Error updating stock: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error updating stock.")),
                    )
                }
            }
        }
        _ => {
            error!(
                "Item with ID '{}' not found in toners or drums.",
                request.item_id
            );
            (
                StatusCode::NOT_FOUND,
                Err(Json("Item not found in toners or drums.")),
            )
        }
    }
}

pub async fn update_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<UpdateMovementRequest>,
) -> impl IntoResponse {
    let movement_id = request.id;
    let new_printer_id = request.printer_id;
    let new_item_id = request.item_id;
    let new_quantity = request.quantity;

    // Not found
    match sqlx::query(r#"SELECT id FROM movements WHERE id = $1;"#)
        .bind(movement_id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(
                r#"UPDATE movements 
                    SET printer_id = $1,
                        item_id = $2,
                        quantity = $3
                    WHERE id = $4;"#,
            )
            .bind(new_printer_id)
            .bind(new_item_id)
            .bind(new_quantity)
            .bind(movement_id)
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    info!("Movement updated! ID: {}", &movement_id);
                    (StatusCode::OK, Ok(Json(movement_id)))
                }
                Err(e) => {
                    error!("Error updating movement: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err(Json("Error updating movement.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Movement ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Movement ID not found.")))
        }
        Err(e) => {
            error!("Error updating movement: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error updating movement.")),
            )
        }
    }
}

pub async fn delete_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM movements WHERE id = $1;"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM movements WHERE id = $1;"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Movement deleted! ID: {}", &request.id);
                    (StatusCode::OK, Ok(Json("Movement deleted!")))
                }
                Err(e) => {
                    error!("Error deleting movement: {}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Ok(Json("Error deleting movement.")),
                    )
                }
            }
        }
        Ok(None) => {
            error!("Movement ID not found.");
            (StatusCode::NOT_FOUND, Err(Json("Movement ID not found")))
        }
        Err(e) => {
            error!("Error deleting movement: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err(Json("Error deleting movement.")),
            )
        }
    }
}
