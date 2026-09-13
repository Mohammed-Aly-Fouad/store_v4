use axum::{Router, routing::get};

use crate::{domain::category::dto::{CategoryResponseDTO, CategoryTree}, state::AppState};










pub fn router() -> Router<AppState> {
    Router::new()
        // .route("/", get(render_categories_page))
        // .route("/", post(create_category))
        // .route("/new", get(render_new_category_page))
        // .route("/{id}", get(render_main_category_details_page))
        // .route("/{id}/edit", get(render_edit_category_form))
        // .route("/{id}/edit", post(edit_category))
        // .route("/{id}/delete", post(delete_category))
        // .route("/search", get(search_categories))
}


pub async fn get_all_categories(state: &AppState) -> Result<Vec<CategoryResponseDTO>, sqlx::Error> {
    sqlx::query_as!(
        CategoryResponseDTO,
        r#"
        SELECT
            c.id,
            c.name_en,
            c.name_ar,
            c.parent_id,
            c.notes,
            c.created_at,
            c.updated_at
        FROM categories c
        ORDER BY c.id DESC;
        "#
    )
    .fetch_all(&state.pool)
    .await
}