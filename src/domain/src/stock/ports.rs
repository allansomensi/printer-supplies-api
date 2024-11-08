use super::models::toner::{DeleteTonerError, DeleteTonerRequest};
use crate::stock::models::toner::CreateTonerError;
use crate::stock::models::toner::{CreateTonerRequest, Toner};

pub trait StockService: Clone + Send + Sync + 'static {
    fn create_toner(
        &self,
        request: &CreateTonerRequest,
    ) -> impl std::future::Future<Output = Result<Toner, CreateTonerError>> + Send;

    fn delete_toner(
        &self,
        request: &DeleteTonerRequest,
    ) -> impl std::future::Future<Output = Result<uuid::Uuid, DeleteTonerError>> + Send;
}

pub trait StockRepository: Send + Sync + Clone + 'static {
    fn create_toner(
        &self,
        request: &CreateTonerRequest,
    ) -> impl std::future::Future<Output = Result<Toner, CreateTonerError>> + Send;

    fn delete_toner(
        &self,
        request: &DeleteTonerRequest,
    ) -> impl std::future::Future<Output = Result<uuid::Uuid, DeleteTonerError>> + Send;
}
