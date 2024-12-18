use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("An error occurred while connecting to the database: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("One or more validation errors occurred: {0}")]
    ValidationError(#[from] validator::ValidationErrors),

    #[error("The provided ID does not correspond to any existing resource.")]
    IdNotFound,

    #[error("A resource with the provided name already exists.")]
    AlreadyExists,

    #[error("No updates were made for the provided ID.")]
    NotModified,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    code: String,
    message: String,
    details: Option<String>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, error_response) = match &self {
            ApiError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: String::from("DATABASE_ERROR"),
                    message: String::from("An unexpected database error occurred."),
                    details: Some(String::from("Please try again later or contact support.")),
                },
            ),
            ApiError::ValidationError(e) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: String::from("VALIDATION_ERROR"),
                    message: String::from("One or more validation errors occurred."),
                    details: Some(e.to_string()),
                },
            ),
            ApiError::IdNotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: String::from("ID_NOT_FOUND"),
                    message: String::from("The provided ID does not exist."),
                    details: Some(String::from(
                        "Please verify that the ID is correct and try again.",
                    )),
                },
            ),
            ApiError::NotModified => (
                StatusCode::NOT_MODIFIED,
                ErrorResponse {
                    code: String::from("NOT_MODIFIED"),
                    message: String::from("No updates were made for the provided ID."),
                    details: Some(String::from(
                        "The provided ID may not exist, or no fields were changed. Please verify the ID and the update values.",
                    )),
                },
            ),
            ApiError::AlreadyExists => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    code: String::from("ALREADY_EXISTS"),
                    message: String::from("A resource with the provided details already exists."),
                    details: Some(String::from("Please choose a different name.")),
                },
            ),
        };

        (status_code, Json(error_response)).into_response()
    }
}
