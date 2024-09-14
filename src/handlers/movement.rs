use chrono::Utc;
use reqwest::StatusCode;
use sqlx::Row;
use std::sync::Arc;
use tracing::{error, info};

use axum::{extract::State, response::IntoResponse, Json};
use uuid::Uuid;

use crate::models::{
    database::AppState,
    movement::{CreateDrumMovementRequest, CreateTonerMovementRequest, Movement},
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
        Some(_) => {
            let printer_id = sqlx::query("SELECT id FROM printers WHERE toner = $1")
                .bind(&request.toner_id)
                .fetch_one(&state.db)
                .await;

            let printer_id: Uuid = printer_id.unwrap().try_get("id").unwrap();

            let new_movement = Movement::new(printer_id, request.toner_id, None, request.quantity);

            sqlx::query(
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
            .unwrap();

            sqlx::query("UPDATE toners SET stock = stock + $1 WHERE id = $2")
                .bind(request.quantity)
                .bind(request.toner_id)
                .execute(&state.db)
                .await
                .unwrap();

            Ok(StatusCode::CREATED)
        }
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_drum_movement(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CreateDrumMovementRequest>,
) -> impl IntoResponse {
    match request.drum_id {
        Some(_) => {
            let printer_id = sqlx::query("SELECT id FROM printers WHERE drum = $1")
                .bind(&request.drum_id)
                .fetch_one(&state.db)
                .await;

            let printer_id: Uuid = printer_id.unwrap().try_get("id").unwrap();

            let new_movement = Movement::new(printer_id, None, request.drum_id, request.quantity);

            sqlx::query(
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
            .unwrap();

            sqlx::query("UPDATE drums SET stock = stock + $1 WHERE id = $2")
                .bind(request.quantity)
                .bind(request.drum_id)
                .execute(&state.db)
                .await
                .unwrap();

            Ok(StatusCode::CREATED)
        }
        None => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}