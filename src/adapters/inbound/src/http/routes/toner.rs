use axum::{routing::post, Router};
use domain::stock::ports::StockService;

use crate::http::{
    handlers::{create_toner::create_toner, delete_toner::delete_toner},
    AppState,
};

pub fn create_routes<SS: StockService>() -> Router<AppState<SS>> {
    Router::new().route("/", post(create_toner::<SS>).delete(delete_toner::<SS>))
}
