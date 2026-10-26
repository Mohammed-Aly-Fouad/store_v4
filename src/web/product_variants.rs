use axum::{Form, Json, Router, extract::{Query, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}, routing::{get, post}};
use sqlx::encode::IsNull::No;

use crate::{domain::{brand::dto::BrandsListDTO, category::dto::FlashParams, home::dto::HomeTemplate, product::dto::ProductListDTO, product_variants::dto::{CreateFormPage, FormDTO, ProductVariantsResponseDTO, ProductVariantsTemplate, VariantSearchQuery, VariantSearchResultsTemplate}}, state::AppState};








pub fn router() -> Router<AppState> {
    Router::new()
    .route("/", get(render_product_variants_page))
    .route("/search", get(search_variants))
    .route("/new", get(render_new_variant_page))
    .route("/", post(create_variant))
}


async fn fetch_all_product_variants(state: &AppState) -> Result<Vec<ProductVariantsResponseDTO>, sqlx::Error> {
    sqlx::query_as!(
        ProductVariantsResponseDTO,
        r#"
        SELECT 
            pv.id,
            p.name_ar AS product_name,
            b.name_ar AS brand_name,
            pv.name_ar, 
            pv.name_en, 
            pv.stock_quantity, 
            pv.reorder_threshold,
            pv.is_active, 
            pv.attr, 
            pv.notes, 
            pv.created_at, 
            pv.updated_at 
        FROM product_variants pv
        JOIN products p ON pv.product_id = p.id
        JOIN brands b ON pv.brand_id = b.id
        ORDER BY pv.id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
}

async fn products_list(
    state: &AppState,
) -> Result<Vec<ProductListDTO>, sqlx::Error> {
    let products = sqlx::query_as!(
        ProductListDTO,
        r#"
        SELECT id, name_ar 
        FROM products p 
        ORDER BY p.id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(products)
}

async fn brands_list(
    state: &AppState,
) -> Result<Vec<BrandsListDTO>, sqlx::Error> {
    let brands = sqlx::query_as!(
        BrandsListDTO,
        r#"
        SELECT id, name_ar 
        FROM brands p 
        ORDER BY p.id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(brands)
}
async fn fetch_variant_by_id(
    state: &AppState,
    id: i64,
) -> Option<ProductVariantsResponseDTO> {
    sqlx::query_as!(
        ProductVariantsResponseDTO,
        r#"
        SELECT 
            pv.id,
            p.name_ar AS product_name,
            b.name_ar AS brand_name,
            pv.name_ar, 
            pv.name_en, 
            pv.stock_quantity, 
            pv.reorder_threshold,
            pv.is_active, 
            pv.attr, 
            pv.notes, 
            pv.created_at, 
            pv.updated_at 
        FROM product_variants pv
        JOIN products p ON pv.product_id = p.id
        JOIN brands b ON pv.brand_id = b.id
        WHERE pv.id = $1
        "#,
        id
    ).fetch_optional(&state.pool)
    .await
    .unwrap_or(None)
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
        Ok(product_variants) => ProductVariantsTemplate {
            product_variants,
            error_message,
            success_message,
            current_page: "product_variants".to_string(),

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

async fn render_new_variant_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> Response {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة الصنف بنجاح".to_string()),
        Some("updated") => Some("تم تعديل الصنف بنجاح".to_string()),
        Some("deleted") => Some("تم حذف الصنف بنجاح".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found") => Some("غير موجود بقاعدة البيانات".to_string()),
        Some("db_err") => Some("خطأ عام بقاعدة البيانات".to_string()),
        Some("can_not_get") => Some("تعذر جلب البيانات الأساسية".to_string()),
        _ => None,
    };

   let products = match products_list(&state).await {
        Ok(p) => p,
        Err(_) => return Redirect::to("/web/product_variants?error=can_not_get").into_response(),
    };

    let brands = match brands_list(&state).await {
        Ok(b) => b,
        Err(_) => return Redirect::to("/web/product_variants?error=can_not_get").into_response(),
    };
    
    CreateFormPage {
        products,
        brands,
        form: FormDTO::default(),
        errors: None,
        success_message,
        error_message,
        current_page: "product_variants".to_string()
    }.into_response()
}


async fn create_variant(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
    Form(mut form): Form<FormDTO>
) -> Response {

    Json(form).into_response()
    // "بسم الله الرحمن الرحيم".into_response()
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
    ProductVariantsResponseDTO,
    r#"
    SELECT 
        pv.id,
        p.name_ar AS product_name,
        b.name_ar AS brand_name,
        pv.name_en, 
        pv.name_ar, 
        pv.stock_quantity, 
        pv.reorder_threshold, 
        pv.is_active,
        pv.attr, 
        pv.notes, 
        pv.created_at, 
        pv.updated_at
    FROM product_variants pv
    JOIN products p ON pv.product_id = p.id
    JOIN brands b ON pv.brand_id = b.id
    WHERE pv.name_en ILIKE $1 
       OR regexp_replace(TRANSLATE(pv.name_ar, ' أإآة', ' أأآه'), '[\u064B-\u0652]', '', 'g') 
          ILIKE regexp_replace(TRANSLATE($1, ' أإآة', ' أأآه'), '[\u064B-\u0652]', '', 'g')
    ORDER BY pv.name_ar ASC
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
