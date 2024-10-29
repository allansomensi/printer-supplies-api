use crate::handlers::status;
use crate::models::status::Status;

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Printer Supplies API",
        description = "A simple REST API using Axum for managing printer supplies, such as toners and drums.",
        contact(name = "Allan Somensi", email = "allansomensidev@gmail.com"),
        license(name = "MIT", identifier = "MIT")
    ),
    paths(
        status::show_status,
    ),
    components(
        schemas(Status)
    ),
    tags(
        (name = "Status", description = "Status endpoints")
    )
)]
pub struct ApiDoc;
