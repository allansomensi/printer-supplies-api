use chrono::Utc;
use reqwest::StatusCode;
use sqlx::Row;
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
    movement::{CreateDrumMovementRequest, CreateTonerMovementRequest, Movement},
    DeleteRequest,
};

pub async fn count_all_movements(State(state): State<Arc<AppState>>) -> Json<i32> {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Json(count)
        }
        Err(e) => {
            error!("Error retrieving movement count: {e}");
            Json(0)
        }
    }
}

pub async fn count_toner_movements(State(state): State<Arc<AppState>>) -> Json<i32> {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements WHERE toner_id IS NOT NULL;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Json(count)
        }
        Err(e) => {
            error!("Error retrieving movement count: {e}");
            Json(0)
        }
    }
}

pub async fn count_drum_movements(State(state): State<Arc<AppState>>) -> Json<i32> {
    let movement_count: Result<(i32,), sqlx::Error> =
        sqlx::query_as(r#"SELECT COUNT(*)::int FROM movements WHERE drum_id IS NOT NULL;"#)
            .fetch_one(&state.db)
            .await;

    match movement_count {
        Ok((count,)) => {
            info!("Successfully retrieved movement count: {}", count);
            Json(count)
        }
        Err(e) => {
            error!("Error retrieving movement count: {e}");
            Json(0)
        }
    }
}

pub async fn search_movement(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Movement>("SELECT * FROM movements WHERE id = $1")
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

pub async fn show_all_movements(State(state): State<Arc<AppState>>) -> Json<Vec<Movement>> {
    match sqlx::query_as(r#"SELECT * FROM movements"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(movements) => {
            info!("Movements listed successfully");
            Json(movements)
        }
        Err(e) => {
            error!("Error listing movements: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn show_toner_movements(State(state): State<Arc<AppState>>) -> Json<Vec<Movement>> {
    match sqlx::query_as(r#"SELECT * FROM movements WHERE toner_id IS NOT NULL;"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(movements) => {
            info!("Toner movements listed successfully");
            Json(movements)
        }
        Err(e) => {
            error!("Error listing toner movements: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn show_drum_movements(State(state): State<Arc<AppState>>) -> Json<Vec<Movement>> {
    match sqlx::query_as(r#"SELECT * FROM movements WHERE drum_id IS NOT NULL;"#)
        .fetch_all(&state.db)
        .await
    {
        Ok(movements) => {
            info!("Drum movements listed successfully");
            Json(movements)
        }
        Err(e) => {
            error!("Error listing drum movements: {e}");
            Json(Vec::new())
        }
    }
}

pub async fn create_toner_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateTonerMovementRequest>,
) -> impl IntoResponse {
    match request.toner_id {
        Some(toner_id) => {
            let printer_id: Uuid = match sqlx::query(r#"SELECT id FROM printers WHERE toner = $1;"#)
                .bind(&toner_id)
                .fetch_one(&state.db)
                .await
                .unwrap()
                .try_get("id")
            {
                Ok(id) => id,
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            };

            let new_movement = Movement::new(printer_id, Some(toner_id), None, request.quantity);

            // Empty quantity
            if new_movement.quantity == 0 {
                error!("Movement quantity cannot be zero.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO movements (id, printer_id, toner_id, drum_id, quantity, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(printer_id)
            .bind(new_movement.toner_id)
            .bind(new_movement.drum_id)
            .bind(new_movement.quantity)
            .bind(Utc::now())
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    sqlx::query("UPDATE toners SET stock = stock + $1 WHERE id = $2")
                        .bind(request.quantity)
                        .bind(toner_id)
                        .execute(&state.db)
                        .await
                        .unwrap();

                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        None => {
            error!("Toner ID not found.");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn create_drum_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateDrumMovementRequest>,
) -> impl IntoResponse {
    match request.drum_id {
        Some(drum_id) => {
            let printer_id: Uuid = match sqlx::query(r#"SELECT id FROM printers WHERE drum = $1;"#)
                .bind(&drum_id)
                .fetch_one(&state.db)
                .await
                .unwrap()
                .try_get("id")
            {
                Ok(id) => id,
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    return StatusCode::INTERNAL_SERVER_ERROR;
                }
            };

            let new_movement = Movement::new(printer_id, None, Some(drum_id), request.quantity);

            // Empty quantity
            if new_movement.quantity == 0 {
                error!("Movement quantity cannot be zero.");
                return StatusCode::BAD_REQUEST;
            }

            match sqlx::query(
                r#"
                INSERT INTO movements (id, printer_id, toner_id, drum_id, quantity, created_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(printer_id)
            .bind(new_movement.toner_id)
            .bind(new_movement.drum_id)
            .bind(new_movement.quantity)
            .bind(Utc::now())
            .execute(&state.db)
            .await
            {
                Ok(_) => {
                    sqlx::query("UPDATE drums SET stock = stock + $1 WHERE id = $2")
                        .bind(request.quantity)
                        .bind(drum_id)
                        .execute(&state.db)
                        .await
                        .unwrap();

                    StatusCode::CREATED
                }
                Err(e) => {
                    error!("Error creating movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        None => {
            error!("Drum ID not found.");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

pub async fn delete_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<DeleteRequest>,
) -> impl IntoResponse {
    match sqlx::query(r#"SELECT id FROM movements WHERE id = $1"#)
        .bind(request.id)
        .fetch_optional(&state.db)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(r#"DELETE FROM movements WHERE id = $1"#)
                .bind(request.id)
                .execute(&state.db)
                .await
            {
                Ok(_) => {
                    info!("Movement deleted! ID: {}", &request.id);
                    StatusCode::OK
                }
                Err(e) => {
                    error!("Error deleting movement: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        }
        Ok(None) => {
            error!("Movement ID not found.");
            StatusCode::NOT_FOUND
        }
        Err(e) => {
            error!("Error deleting movement: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
