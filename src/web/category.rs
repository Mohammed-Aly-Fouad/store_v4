use axum::{Router, extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::get};

use crate::{domain::category::dto::{CategoryResponseDTO, CategorySearchQuery, CategorySearchResultsTemplate, CategoryTemplate, CategoryTree, FlashParams}, state::AppState};










pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page))
        // .route("/", post(create_category))
        // .route("/new", get(render_new_category_page))
        // .route("/{id}", get(render_main_category_details_page))
        // .route("/{id}/edit", get(render_edit_category_form))
        // .route("/{id}/edit", post(edit_category))
        // .route("/{id}/delete", post(delete_category))
        .route("/search", get(search_categories))
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

#[axum::debug_handler]
pub async fn render_categories_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> CategoryTemplate {
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
    match get_all_categories(&state).await {
        Ok(all_categories) => CategoryTemplate {
            category_tree: CategoryTree::build_tree(all_categories),
            error_message: None,
            success_message: success_message,
            current_page: "categories".to_string(),
        },
        Err(err) => {
            tracing::error!("Failed to fetch categories: {:?}", err);
            CategoryTemplate {
                category_tree: Vec::new(),
                error_message: Some("Failed to load categories.".to_string()),
                success_message: None,
                current_page: "categories".to_string(),
            }
        }
    }
}



// ============================================================================
// HANDLERS: LIVE SEARCH
// ============================================================================
pub async fn search_categories(
    State(state): State<AppState>,
    Query(query): Query<CategorySearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(CategorySearchResultsTemplate {
            categories: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

let categories = sqlx::query_as!(
    CategoryResponseDTO,
    r#"
    SELECT id, name_en, name_ar, parent_id, notes, created_at, updated_at
    FROM categories
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
    tracing::error!("Failed to execute category search query: {:?}", err);
    StatusCode::INTERNAL_SERVER_ERROR
})?;

    Ok(CategorySearchResultsTemplate {
        categories,
        query: q.to_string(),
    })
}