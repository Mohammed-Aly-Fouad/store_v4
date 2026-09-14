use axum::{Json, Router, extract::{Path, Query, State}, http::StatusCode, response::{IntoResponse, Response}, routing::get};

use crate::{domain::home::dto::{CategorySearchQuery, CategorySearchResultsTemplate, FlashParams, HomeTemplate}, state::AppState};












pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(home))
        // .route("/{id}/branch", get(render_branch_page))
        // .route("/", post(create_category))
        // .route("/new", get(render_new_category_page))
        // .route("/{id}", get(render_main_category_details_page))
        // .route("/{id}/edit", get(render_edit_category_form))
        // .route("/{id}/edit", post(edit_category))
        // .route("/{id}/delete", post(delete_category))
        // .route("/search", get(search_categories))
}

async fn home (Query(params): Query<FlashParams>) -> Response {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة الفئة بنجاح".to_string()),
        Some("updated") => Some("تم تعديل الفئة بنجاح".to_string()),
        Some("deleted") => Some("تم حذف الفئة بنجاح".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("db_error") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };
    HomeTemplate {
        success_message,
        error_message,
        current_page: "home".to_string(),
    }.into_response()
}