use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::http::{
    responses::{ApiError, ApiSuccess},
    AppState,
};
use domain::stock::{
    models::toner::{DeleteTonerError, DeleteTonerRequest, Toner, TonerIdEmptyError},
    ports::StockService,
};

impl From<DeleteTonerError> for ApiError {
    fn from(e: DeleteTonerError) -> Self {
        match e {
            DeleteTonerError::NotFound { id } => {
                Self::NotFound(format!("Toner with id {id} not found"))
            }
            DeleteTonerError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError(String::from("Internal server error"))
            }
        }
    }
}

impl From<ParseDeleteTonerHttpRequestError> for ApiError {
    fn from(e: ParseDeleteTonerHttpRequestError) -> Self {
        let message = match e {
            ParseDeleteTonerHttpRequestError::Id(_) => String::from("Toner id cannot be empty"),
        };

        Self::UnprocessableEntity(message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeleteTonerRequestBody {
    id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeleteTonerResponseData {
    id: String,
}

impl From<&Uuid> for DeleteTonerResponseData {
    fn from(toner_id: &Uuid) -> Self {
        Self {
            id: toner_id.to_string(),
        }
    }
}

impl From<&Toner> for DeleteTonerResponseData {
    fn from(toner: &Toner) -> Self {
        Self {
            id: toner.id().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct DeleteTonerHttpRequestBody {
    id: String,
}

#[derive(Debug, Clone, Error)]
enum ParseDeleteTonerHttpRequestError {
    #[error(transparent)]
    Id(#[from] TonerIdEmptyError),
}

impl DeleteTonerHttpRequestBody {
    fn try_into_domain(self) -> Result<DeleteTonerRequest, ParseDeleteTonerHttpRequestError> {
        let id = Uuid::parse_str(&self.id).unwrap();
        Ok(DeleteTonerRequest::new(id))
    }
}

pub async fn delete_toner<SS: StockService>(
    State(state): State<AppState<SS>>,
    Json(body): Json<DeleteTonerHttpRequestBody>,
) -> Result<ApiSuccess<DeleteTonerResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .toner_service
        .delete_toner(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref id: Uuid| ApiSuccess::new(StatusCode::OK, id.into()))
}
