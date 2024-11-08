use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::http::responses::{ApiError, ApiSuccess};
use crate::http::AppState;
use domain::stock::models::toner::CreateTonerError;
use domain::stock::models::toner::{CreateTonerRequest, Toner, TonerName, TonerNameEmptyError};
use domain::stock::ports::StockService;

impl From<CreateTonerError> for ApiError {
    fn from(e: CreateTonerError) -> Self {
        match e {
            CreateTonerError::Duplicate { name } => {
                Self::UnprocessableEntity(format!("Toner with name {} already exists", name))
            }
            CreateTonerError::Unknown(cause) => {
                tracing::error!("{:?}\n{}", cause, cause.backtrace());
                Self::InternalServerError(String::from("Internal server error"))
            }
        }
    }
}

impl From<ParseCreateTonerHttpRequestError> for ApiError {
    fn from(e: ParseCreateTonerHttpRequestError) -> Self {
        let message = match e {
            ParseCreateTonerHttpRequestError::Name(_) => String::from("Toner name cannot be empty"),
        };

        Self::UnprocessableEntity(message)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateTonerRequestBody {
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CreateTonerResponseData {
    id: String,
}

impl From<&Toner> for CreateTonerResponseData {
    fn from(toner: &Toner) -> Self {
        Self {
            id: toner.id().to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CreateTonerHttpRequestBody {
    name: String,
}

#[derive(Debug, Clone, Error)]
enum ParseCreateTonerHttpRequestError {
    #[error(transparent)]
    Name(#[from] TonerNameEmptyError),
}

impl CreateTonerHttpRequestBody {
    fn try_into_domain(self) -> Result<CreateTonerRequest, ParseCreateTonerHttpRequestError> {
        let name = TonerName::new(&self.name)?;
        Ok(CreateTonerRequest::new(name))
    }
}

pub async fn create_toner<BS: StockService>(
    State(state): State<AppState<BS>>,
    Json(body): Json<CreateTonerHttpRequestBody>,
) -> Result<ApiSuccess<CreateTonerResponseData>, ApiError> {
    let domain_req = body.try_into_domain()?;
    state
        .toner_service
        .create_toner(&domain_req)
        .await
        .map_err(ApiError::from)
        .map(|ref toner| ApiSuccess::new(StatusCode::CREATED, toner.into()))
}
