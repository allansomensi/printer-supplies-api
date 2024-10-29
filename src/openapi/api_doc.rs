use crate::handlers::{status, supplies::drum};
use crate::models::status::Status;
use crate::models::supplies::drum::Drum;

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        title = "Printer Supplies API",
        description = "A simple REST API using Axum for managing printer supplies, such as toners and drums.",
        contact(name = "Allan Somensi", email = "allansomensidev@gmail.com"),
        license(name = "MIT", identifier = "MIT")
    ),
    paths(
        // Status
        status::show_status,

        // Drum
        drum::count_drums,
        drum::search_drum,
        drum::show_drums,
        drum::create_drum,
        drum::update_drum,
        drum::delete_drum,
    ),
    components(
        schemas(Status, Drum)
    ),
    tags(
        (name = "Status", description = "Status endpoints"),
        (name = "Drums", description = "Drums endpoints"),
    )
)]
pub struct ApiDoc;
