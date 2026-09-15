use std::default;

use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect, Response};
use sqlx::encode::IsNull::No;

use crate::domain::{category, product};
use crate::domain::product::dto::{CreateFormPage, FlashParams, FormDTO, FormErrors, ProductResponseDTO, ProductSearchQuery, ProductSearchResultsTemplate, ProductTemplate, ProductWithCategoryDTO, SubCategoriesList};
use crate::state::{self, AppState};

use crate::web::category::get_all_categories;


pub fn router() -> Router<AppState> {
    
    Router::new()
    .route("/", get(render_products_page))
    .route("/", post(create_product))
    .route("/new", get(render_new_product_page))
    .route("/search", get(search_products))
}






pub async fn get_products_with_full_category(
    state: &AppState,
) -> Result<Vec<ProductWithCategoryDTO>, sqlx::Error> {
    let products = sqlx::query_as!(
        ProductWithCategoryDTO,
        r#"
        SELECT 
            p.id,
            p.category_id,
            p.name_ar,
            p.name_en,
            p.notes,
            p.created_at,
            p.updated_at,
            c.name_ar AS category_name_ar,
            parent.name_ar AS parent_category_name_ar
        FROM products p
        INNER JOIN categories c ON p.category_id = c.id
        LEFT JOIN categories parent ON c.parent_id = parent.id
        ORDER BY p.id DESC
        "#
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(products)
}


async fn get_sub_categories_list(
    state: &AppState
) -> Result<Vec<SubCategoriesList>, sqlx::Error> {
    let sub_categories_list = sqlx::query_as!(
        SubCategoriesList,
        r#"
        SELECT
    c.id,
    c.parent_id,
    c.name_en,
    c.name_ar,
    c.notes,
    c.created_at,
    c.updated_at,
    p.name_en AS parent_name_en,
    p.name_ar AS parent_name_ar
FROM public.categories c
INNER JOIN public.categories p ON c.parent_id = p.id
WHERE c.parent_id IS NOT NULL;
        "#
    ).fetch_all(&state.pool)
    .await?;
Ok(sub_categories_list)
}


async fn render_products_page(
    State(state): State<AppState>,
    Query(params): Query<FlashParams>,
) -> Response {
    let success_message = match params.action.as_deref() {
        Some("created") => Some("تم إضافة المنتج الصنف بنجاح".to_string()),
        Some("updated") => Some("تم تعديل الصنف بنجاح".to_string()),
        Some("deleted") => Some("تم حذف الصنف بنجاح".to_string()),
        _ => None,
    };

    let error_message = match params.error.as_deref() {
        Some("not_found")=> Some("غير موجود بقاعدة البيانات".to_string()),
        Some("db_err") => Some("خطأ عام بقاعدة البيانات".to_string()),
        _ => None,
    };

    let products = match get_products_with_full_category(&state).await {
        Ok(products) => products,
        Err(err) => {
            tracing::error!("فشل جلب الفئات: {:#?}", err);
            return Redirect::to("/web/products?error=server_error").into_response();
        }
    };



   return  ProductTemplate {
    products,
    error_message,
    success_message,
    current_page: "products".to_string(),
    }.into_response()

}

async fn render_new_product_page(
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
        _ => None,
    };

    match get_sub_categories_list(&state).await {
        Ok(categories_list) => CreateFormPage {
            form: FormDTO::default(),
            categories_list,
            errors: None,
            current_page: "products".to_string(),
            success_message, // تم تمرير المتغير الصحيح
            error_message,   // تم تمرير المتغير الصحيح
        }
        .into_response(),

        Err(err) => CreateFormPage {
            form: FormDTO::default(),
            categories_list: vec![],
            errors: None,
            current_page: "products".to_string(),
            success_message: None,
            error_message: Some(format!("فشل تحميل الفئات: {}", err)),
        }
        .into_response(),
    }
}

async fn create_product(
    State(state): State<AppState>,
    Form(mut form): Form<FormDTO>,
) -> Response {
    let categories_list = match get_sub_categories_list(&state).await {
        Ok(list) => list,
        Err(err) => {
            tracing::error!("فشل جلب الفئات: {:#?}", err);
            return Redirect::to("/web/products?error=db_err").into_response();
        }
    };

    form.sanitize();

    if let Err(err) = form.validate(&categories_list) {
        // tracing::error!("فشل التحقق من صحة النموذج: {:#?}", err);
        tracing::error!(?err);
        return CreateFormPage {
            categories_list,
            form,
            errors: Some(err),
            error_message: Some("من فضلك عدل الأخطاء لاستكمال التسجيل".to_string()),
            success_message: None,
            current_page: "products".to_string(),
        }
        .into_response();
    }

    let insert_result = sqlx::query!(
        r#"
        INSERT INTO products (category_id, name_en, name_ar, notes) 
        VALUES (
            (SELECT id FROM categories WHERE categories.name_ar = $1 LIMIT 1),
            $2, $3, $4
        )
        "#,
        &form.category,
        &form.name_en,
        &form.name_ar,
        form.notes.as_deref(),
    )
    .execute(&state.pool)
    .await;

    match insert_result {
        Ok(_) => Redirect::to("/web/products?action=created").into_response(),

        // خطأ تكرار القيمة الفريدة (Unique Constraint Violation)
        Err(sqlx::Error::Database(db_err)) if db_err.code().as_deref() == Some("23505") => {
            let err_msg = format!("الصنف \"{}\" بالعربية أو بالإنجليزية مسجل بالفعل", form.name_ar);
            CreateFormPage {
                form,
                categories_list,
                error_message: Some(err_msg),
                success_message: None,
                errors: None,
                current_page: "products".to_string(),
            }
            .into_response()
        }


        Err(err) => {
            tracing::error!("خطأ أثناء حفظ الصنف: {:#?}", err);
            CreateFormPage {
                form,
                categories_list,
                error_message: Some("حدث خطأ غير متوقع أثناء الحفظ في قاعدة البيانات".to_string()),
                success_message: None,
                errors: None,
                current_page: "products".to_string(),
            }
            .into_response()
        }
       
    }
}





// ============================================================================
// HANDLERS: LIVE SEARCH
// ============================================================================
pub async fn search_products(
    State(state): State<AppState>,
    Query(query): Query<ProductSearchQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    let q = query.q.trim();

    // إرجاع استجابة فارغة فوراً إن كان الاستعلام خالياً
    if q.is_empty() {
        return Ok(ProductSearchResultsTemplate {
            products: vec![],
            query: String::new(),
        });
    }

    // إعداد نمط البحث غير حساس للحالة (Case-Insensitive) للغتين العربية والإنجليزية
    let search_pattern = format!("%{}%", q);

let products = sqlx::query_as!(
    ProductResponseDTO,
    r#"
    SELECT id, category_id, name_en, name_ar, notes, created_at, updated_at
    FROM products
    WHERE name_ar ILIKE $1 
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

    Ok(ProductSearchResultsTemplate {
        products,
        query: q.to_string(),
    })
}