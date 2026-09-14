use axum::{Form, Json, Router, extract::{Path, Query, State}, http::StatusCode, response::{IntoResponse, Redirect, Response}, routing::{get, post}};
use urlencoding::encode;

use crate::{domain::{category::dto::{BranchTemplate, CategoryFormDTO, CategoryResponseDTO, CategorySearchQuery, CategorySearchResultsTemplate, CategoryTemplate, CategoryTree, CreateFormPage, FlashParams}, home::dto::HomeTemplate}, state::AppState};










pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(render_categories_page))
        .route("/{id}/branch", get(render_branch_page))
        .route("/new", get(render_new_category_page))
        .route("/", post(create_category))
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

async fn get_category_branch(
    state: &AppState,
    id: i64,
) -> Result<Vec<CategoryResponseDTO>, sqlx::Error> {
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
WHERE c.id = $1 OR c.parent_id = $1
ORDER BY c.parent_id IS NOT NULL, c.id DESC;
        "#,
        id
    )
    .fetch_all(&state.pool)
    .await
}



#[axum::debug_handler]
pub async fn render_categories_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> Response {
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
            error_message,
            success_message,
            current_page: "categories".to_string(),
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


#[axum::debug_handler]
pub async fn render_branch_page(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<FlashParams>,
) ->  Response {
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
    
   match get_category_branch(&state, id).await {
        Ok(branch) => BranchTemplate {
            branch_id: id,
            category_tree: CategoryTree::build_tree(branch),
            error_message,
            success_message: success_message,
            current_page: "categories".to_string(),
        }.into_response(),
        Err(err) => {
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
            tracing::error!("Failed to fetch categories: {:?}", err);

         let error_string = custom_db_error.to_string();
let msg = encode(&error_string);
return Redirect::to(&format!("/web/?error_message={}", msg)).into_response();
        },
    }
}




#[axum::debug_handler]
async fn render_new_category_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> impl IntoResponse {
    match get_all_categories(&state).await {
        Ok(all_categories) => CreateFormPage {
            category_tree: CategoryTree::build_tree(all_categories),
            form: CategoryFormDTO::default(),
            errors: None,
            current_page: "categories".to_string(),
            success_message: None,
            error_message: None,
        },

        Err(err) => CreateFormPage {
            category_tree: Vec::new(),
            form: CategoryFormDTO::default(),
            errors: None,
            current_page: "categories".to_string(),
            success_message: None,
            error_message: Some(err.to_string()),
        },
    }
}

async fn create_category(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
    Form(mut form): Form<CategoryFormDTO>,
) -> Response {
    let all_categories = match get_all_categories(&state).await {
        Ok(categories) => categories,
        Err(err) => {
            tracing::error!("فشل جلب الفئات: {:#?}", err);
            return Redirect::to("/web/categories?error=server_error").into_response();
        }
    };

    let category_tree = CategoryTree::build_tree(all_categories);
    form.sanitize();
    match form.validate(&category_tree) {
        Err(err) => {
            tracing::error!("فشل التحقق من صحة النموذج: {:#?}", err);
            return CreateFormPage {
                form,
                category_tree,
                error_message: Some("من فضلك عدل الأخطاء لاستكمال التسجيل".to_string()),
                success_message: None,
                errors: Some(err),
                current_page: "categories".to_string(),
            }
            .into_response();
        }
        Ok(_) => {
            let insert_result = sqlx::query!(
                r#"
                INSERT INTO categories (name_en, name_ar, parent_id, notes)
                VALUES (
                $1,
                $2,
                (SELECT id FROM categories WHERE name_ar = $3 LIMIT 1),
                 $4
                 )
                 "#,
                &form.name_en,
                &form.name_ar,
                form.parent_name.as_deref(),
                form.notes.as_deref(),
            )
            .execute(&state.pool)
            .await;

            match insert_result {
                Ok(_) => Redirect::to("/web/categories?action=created").into_response(),
                Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
                    let err_msg = format!("الفئة \"{}\" مسجلة بالفعل", form.name_ar);
                    return CreateFormPage {
                        form,
                        category_tree,
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
                        category_tree,
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