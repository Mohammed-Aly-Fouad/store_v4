pub mod category;
pub mod home;
pub mod product;
pub mod brand;

use crate::{state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(home::router())
        .nest("/categories", category::router())
        .nest("/products", product::router())
        .nest("/brands", brand::router())
}