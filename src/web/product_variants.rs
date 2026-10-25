use axum::{Router, extract::{Query, State}, http::StatusCode, response::{IntoResponse, Response}, routing::get};
use sqlx::encode::IsNull::No;

use crate::{domain::{category::dto::FlashParams, home::dto::HomeTemplate, product_variants::dto::{ProductVariantResponseDTO, ProductVariantTemplate, VariantSearchQuery, VariantSearchResultsTemplate}}, state::AppState};








pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_product_variants_page))
    .route("/search", get(search_variants))
}


async fn fetch_all_product_variants(state: &AppState) -> Result<Vec<ProductVariantResponseDTO>, sqlx::Error> {
    sqlx::query_as!(
        ProductVariantResponseDTO,
        r#"
        SELECT id, product_id, brand_id, name_ar, name_en, sku, barcode, shelf_location, stock_quantity, reorder_threshold, is_active, attr, notes, created_at, updated_at FROM product_variants ORDER BY id DESC
        "#
    ).fetch_all(&state.pool)
    .await
}



pub async fn render_product_variants_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> Response {

   const MSG_CREATED: &str = "تم إضافة متغير المنتج بنجاح.";
const MSG_UPDATED: &str = "تم تعديل متغير المنتج بنجاح.";
const MSG_DELETED: &str = "تم حذف متغير المنتج بنجاح.";

let success_message = match params.action.as_deref() {
    Some("created") => Some(MSG_CREATED.to_string()),
    Some("updated") => Some(MSG_UPDATED.to_string()),
    Some("deleted") => Some(MSG_DELETED.to_string()),
    _ => None,
};
    let error_message = match params.error.as_deref() {
         Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("db_error") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };

    match fetch_all_product_variants(&state).await {
        Ok(products_variants) => ProductVariantTemplate {
            products_variants,
            error_message,
            success_message,
            current_page: "products_variants".to_string(),

        }.into_response(),

        Err(err) => {
            // Match specific sqlx::Error variants to customize log & UI messages
            let custom_db_error = match &err {
                sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => {
                    tracing::error!(error = %err, "Database pool connection failed/timed out");
                    "تعذر الاتصال بقاعدة البيانات (انتهت المهلة)."
                }
                sqlx::Error::Io(_) => {
                    tracing::error!(error = %err, "Database network/IO error");
                    "تعذر الوصول إلى خادم قاعدة البيانات."
                }
                sqlx::Error::RowNotFound => {
                    "البيانات المطلوبة غير موجودة."
                }
                _ => {
                    tracing::error!(error = %err, "Unhandled SQLx error");
                    "حدث خطأ غير متوقع في قاعدة البيانات."
                }
            };

            HomeTemplate {
                error_message: Some(custom_db_error.to_string()),
                success_message: None,
                current_page: "categories".to_string(),
            }.into_response()
        }
    }
}



// ============================================================================
// HANDLERS: LIVE SEARCH
// ============================================================================
pub async fn search_variants(
    State(state): State<AppState>,
    Query(query): Query<VariantSearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(VariantSearchResultsTemplate {
            variants: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

let variants = sqlx::query_as!(
    ProductVariantResponseDTO,
    r#"
    SELECT id, product_id, brand_id, name_en, name_ar, sku, barcode, shelf_location, stock_quantity, reorder_threshold, is_active, attr, notes, created_at, updated_at
    FROM product_variants
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
    tracing::error!("Failed to execute product search query: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
})?;

    Ok(VariantSearchResultsTemplate {
        variants,
        query: q.to_string(),
    })
}
