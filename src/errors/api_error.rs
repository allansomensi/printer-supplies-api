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

    #[error("The name field must not be empty.")]
    EmptyName,

    #[error("The provided name is too short. It must be at least 4 characters long.")]
    NameTooShort,

    #[error("The provided name is too long. It must not exceed 20 characters.")]
    NameTooLong,

    #[error("A resource with the provided name already exists.")]
    AlreadyExists,

    #[error("An unknown error occurred. Please try again later.")]
    Unknown,
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
            ApiError::EmptyName => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: String::from("EMPTY_NAME"),
                    message: String::from("The name cannot be empty."),
                    details: Some(String::from(
                        "Please provide a name with at least 4 characters.",
                    )),
                },
            ),
            ApiError::NameTooShort => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: String::from("NAME_TOO_SHORT"),
                    message: String::from("The provided name is too short."),
                    details: Some(String::from("The name must be at least 4 characters long.")),
                },
            ),
            ApiError::NameTooLong => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: String::from("NAME_TOO_LONG"),
                    message: String::from("The provided name is too long."),
                    details: Some(String::from("The name must not exceed 20 characters.")),
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
            ApiError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: String::from("UNKNOWN_ERROR"),
                    message: String::from("An unknown error occurred."),
                    details: Some(String::from("Please try again later or contact support.")),
                },
            ),
        };

        (status_code, Json(error_response)).into_response()
    }
}
