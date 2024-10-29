use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::api_doc::ApiDoc;

pub fn swagger_routes() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}