use super::models::toner::{DeleteTonerError, DeleteTonerRequest};
use crate::stock::models::toner::CreateTonerError;
use crate::stock::models::toner::{CreateTonerRequest, Toner};
use crate::stock::ports::{StockRepository, StockService};

#[derive(Debug, Clone)]
pub struct Service<R>
where
    R: StockRepository,
{
    repository: R,
}

impl<R> Service<R>
where
    R: StockRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R> StockService for Service<R>
where
    R: StockRepository,
{
    async fn create_toner(&self, request: &CreateTonerRequest) -> Result<Toner, CreateTonerError> {
        self.repository.create_toner(request).await
    }

    async fn delete_toner(
        &self,
        request: &DeleteTonerRequest,
    ) -> Result<uuid::Uuid, DeleteTonerError> {
        self.repository.delete_toner(request).await
    }
}
