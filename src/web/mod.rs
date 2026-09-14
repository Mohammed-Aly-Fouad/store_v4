pub mod category;
pub mod home;
pub mod product;

use crate::{state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(home::router())
        .nest("/categories", category::router())
        .nest("/products", product::router())
}