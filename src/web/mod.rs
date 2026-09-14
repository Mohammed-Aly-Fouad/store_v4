pub mod category;
pub mod home;

use crate::{state::AppState};
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(home::router())
        .nest("/categories", category::router())
}