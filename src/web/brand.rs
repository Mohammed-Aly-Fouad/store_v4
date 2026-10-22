use axum::{Router, extract::{Path, Query, State}, routing::get};
use serde::Deserialize;

use crate::{domain::brand::dto::{BrandResponseDTO, BrandsTemplate}, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_brands_page))
    // .route("/", post(create_brand))
    // .route("/create", get(render_create_page))
    // .route("/{id}/edit", get(render_edit_page))
    // .route("/{id}/edit", post(edit_brand))
    // .route("/{id}/delete", post(delete_brand))
    // .route("/search", get(search_brands))

    }

    async fn fetch_all_brands(state: &AppState) -> Vec<BrandResponseDTO> {
    sqlx::query_as!(
        BrandResponseDTO,
        r#"
        SELECT id, name_en, name_ar, notes, created_at, updated_at 
        FROM brands 
        ORDER BY id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default()
}

pub async fn render_brands_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> BrandsTemplate {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة البراند بنجاح".to_string()),
        Some("updated") => Some("تم تعديل البراند بنجاح".to_string()),
        Some("deleted") => Some("تم حذف البراند بنجاح".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("db_error") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };
    BrandsTemplate {
        brands: fetch_all_brands(&state).await,
        error_message: error_message,
        success_message,
        current_page: "brands".to_string(),
    }
}


// ---------------------------------------------------------------------------
// 4.1 Custom Askama Filters
// ---------------------------------------------------------------------------

// We name it filter so no need to "use filters"
pub mod filters {
    use askama::Values;

    /// Extract the capitalized initial character of a brand name for UI avatar circles.
    #[askama::filter_fn]
    pub fn first_letter(name: &str, _values: &dyn Values) -> askama::Result<String> {
        Ok(name
            .trim()
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "؟".to_string()))
    }

    /// Generates a deterministic hex color code based on the brand string hash.
    /// Ensures the same brand always gets the exact same avatar background color across renders.
    #[askama::filter_fn]
    pub fn initial_color(name: &str, _values: &dyn Values) -> askama::Result<String> {
        const PALETTE: [&str; 6] = [
            "#0E7C66", "#2563EB", "#D97706", "#7C3AED", "#DB2777", "#0891B2",
        ];
        let sum: u32 = name.bytes().map(|b| b as u32).sum();
        Ok(PALETTE[sum as usize % PALETTE.len()].to_string())
    }
}


// ============================================================================
// FLASH MESSAGES & QUERY PARAMS
// ============================================================================

/// Query parameters used to carry one-time "Flash Messages" across HTTP Redirects.

#[derive(Debug, Deserialize)]
pub struct FlashParams {
    pub action: Option<String>,
    pub error: Option<String>
}