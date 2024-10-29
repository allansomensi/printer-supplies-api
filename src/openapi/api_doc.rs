use crate::models::brand::Brand;
use crate::models::status::Status;
use crate::models::supplies::drum::Drum;
use crate::{
    handlers::{
        brand, status,
        supplies::{drum, toner},
    },
    models::supplies::toner::Toner,
};

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

        // Toner
        toner::count_toners,
        toner::search_toner,
        toner::show_toners,
        toner::create_toner,
        toner::update_toner,
        toner::delete_toner,

        // Drum
        drum::count_drums,
        drum::search_drum,
        drum::show_drums,
        drum::create_drum,
        drum::update_drum,
        drum::delete_drum,

        // Brands
        brand::count_brands,
        brand::search_brand,
        brand::show_brands,
        brand::create_brand,
        brand::update_brand,
        brand::delete_brand,

    ),
    components(
        schemas(Status, Drum, Toner, Brand)
    ),
    tags(
        (name = "Status", description = "Status endpoints"),
        (name = "Toners", description = "Toners endpoints"),
        (name = "Drums", description = "Drums endpoints"),
        (name = "Brands", description = "Brands endpoints"),
    )
)]
pub struct ApiDoc;
