use axum::{Form, Json, Router, extract::{Path, Query, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}, routing::{get, post}};
use serde::Deserialize;

use crate::{domain::brand::dto::{BrandResponseDTO, BrandSearchQuery, BrandSearchResultsTemplate, BrandsTemplate, CreateFormPage, FormDTO}, state::{self, AppState}};

pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_brands_page))
    .route("/", post(create_brand))
    .route("/new", get(render_new_brand_page))
    // .route("/{id}/edit", get(render_edit_page))
    // .route("/{id}/edit", post(edit_brand))
    // .route("/{id}/delete", post(delete_brand))
    .route("/search", get(search_brands))

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

pub async fn search_brands(
    State(state): State<AppState>,
    Query(query): Query<BrandSearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(BrandSearchResultsTemplate {
            brands: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

let brands = sqlx::query_as!(
    BrandResponseDTO,
    r#"
    SELECT id, name_en, name_ar, notes, created_at, updated_at
    FROM brands
    WHERE name_en ILIKE $1 
       OR regexp_replace(TRANSLATE(name_ar, 'أإآىة', 'ااايه'), '[\u064B-\u0652]', '', 'g') 
          ILIKE regexp_replace(TRANSLATE($1, 'أإآىة', 'ااايه'), '[\u064B-\u0652]', '', 'g')
    ORDER BY name_ar ASC
    LIMIT 10
    "#,
    search_pattern
)
.fetch_all(&state.pool)
.await
.map_err(|err| {
    tracing::error!("Failed to execute brand search query: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
})?;

    Ok(BrandSearchResultsTemplate {
        brands,
        query: q.to_string(),
    })
}

async fn render_new_brand_page(
) -> impl IntoResponse {
    CreateFormPage {
        form: FormDTO::default(),
        errors: None,
        current_page: "brands".to_string(),
        success_message: None,
        error_message: None,
    }
}

async fn create_brand(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
    Form(mut form): Form<FormDTO>,
) -> impl IntoResponse {
    form.sanitize();

   match form.validate() {
       Err(err) => {
        return CreateFormPage {
            form,
             error_message: Some("من فضلك عدل الأخطاء لاستكمال التسجيل".to_string()),
                success_message: None,
                errors: Some(err),
                current_page: "brands".to_string(),
        }.into_response();
       }

       Ok(_) => {
        let insert_result = sqlx::query!(
                r#"
                INSERT INTO brands (name_en, name_ar, notes)
                VALUES (
                $1,
                $2,
                 $3
                 )
                 "#,
                &form.name_en,
                &form.name_ar,
                form.notes.as_deref(),
            )
            .execute(&state.pool)
            .await;

        match insert_result {
                Ok(_) => Redirect::to("/web/brands?action=created").into_response(),
                Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                    let err_msg = format!("الماركة \"{}\" مسجلة بالفعل", form.name_ar);
                    return CreateFormPage {
                        form,
                        error_message: Some(err_msg),
                        success_message: None,
                        errors: None,
                        current_page: "categories".to_string(),
                    }
                    .into_response();
                }
                Err(err) => {
                    tracing::error!("خطأ عام: {:#?}", err);
                    return CreateFormPage {
                        form,
                        error_message: None,
                        success_message: None,
                        errors: None,
                        current_page: "categories".to_string(),
                    }
                    .into_response();
                }
            }
       }
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