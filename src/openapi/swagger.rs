use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use super::api_doc::ApiDoc;

pub fn swagger_route() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}
