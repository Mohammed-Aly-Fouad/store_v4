use std::collections::HashMap;

use askama::Template;
use askama_web::WebTemplate;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use sqlx::FromRow;

use crate::domain::{brand::dto::{BrandResponseDTO, BrandsListDTO}, product::dto::ProductListDTO};

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

#[derive(Debug, Clone, Serialize, FromRow)] // تم إزالة Deserialize هنا
pub struct ProductVariantsResponseDTO {
    pub id: i64,
    pub product_name: String,
    pub brand_name: String,
    pub name_ar: String,
    pub name_en: String,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
    pub is_active: bool,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub attr: sqlx::types::Json<std::collections::HashMap<String, String>>,
}


// #[derive(Default, Debug, Serialize)]
// pub struct FormErrors {
//     pub product_name: Option<String>,
//     pub brand_name: Option<String>,
//     pub name_ar: Option<String>,
//     pub name_en: Option<String>,
//     pub stock_quantity: Option<String>,
//     pub reorder_threshold: Option<String>,
//     pub is_active: Option<String>,
//     pub notes: Option<String>,
// }

// impl FormErrors {
//     pub fn has_errors(&self) -> bool {
//             self.name_ar.is_some()
//             || self.name_en.is_some()
//             || self.stock_quantity.is_some()
//             || self.reorder_threshold.is_some()
//             || self.is_active.is_some()
//             || self.notes.is_some()
//     }
// }

// #[derive(Debug, Deserialize, Serialize)]
// #[serde(default)]
// pub struct FormDTO {
//     pub product_id: i64,
//     pub brand_id: i64,
//     pub name_ar: String,
//     pub name_en: String,
//     pub stock_quantity: i32,
//     pub reorder_threshold: i32,
//     pub is_active: bool,
//     #[serde(default, deserialize_with = "empty_string_as_none")]
//     pub notes: Option<String>,
//     // أضف هذه الحقول لاستقبال المفاتيح والقيم القادمة من الفورم
//     pub attribute_keys: Vec<String>,
//     pub attribute_values: Vec<String>,
     
// }

// impl FormDTO {
//     pub fn sanitize(&mut self) {
//         self.name_ar = self.name_ar.trim().to_string();
//         self.name_en = self.name_en.trim().to_string();
//         self.notes = self
//             .notes
//             .as_deref()
//             .map(str::trim)
//             .filter(|s| !s.is_empty())
//             .map(ToString::to_string);
//     }

//    // دالة جديدة لتجميع الخصائص في شكل JSON Object
//     pub fn to_attributes_json(&self) -> serde_json::Value {
//         let mut map = serde_json::Map::new();
        
//         for (key, value) in self.attribute_keys.iter().zip(self.attribute_values.iter()) {
//             let trimmed_key = key.trim();
//             let trimmed_val = value.trim();
            
//             if !trimmed_key.is_empty() {
//                 map.insert(
//                     trimmed_key.to_string(), 
//                     serde_json::Value::String(trimmed_val.to_string())
//                 );
//             }
//         }
        
//         serde_json::Value::Object(map)
//     }
// }

// impl Default for FormDTO {
//     fn default() -> Self {
//         Self {
//           product_id: 0,        // تعيين قيمتها كـ i64 افتراضياً
//             brand_id: 0,          // تعيين قيمتها كـ i64 افتراضياً
//             name_ar: String::new(),
//             name_en: String::new(),
//             stock_quantity: 0,
//             reorder_threshold: 0,
//             is_active: true,
//             notes: None,
//             attribute_keys: Vec::new(),   // أضف هذا السطر
//             attribute_values: Vec::new(), // أضف هذا السطر أيضاً
// }
//     }
// }
// // ############ Templates ##################
#[derive(Template, WebTemplate)]
#[template(path = "product_variants/index.html")]
pub struct ProductVariantsTemplate {
    pub product_variants: Vec<ProductVariantsResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,
}


// #[derive(Template, WebTemplate)]
// #[template(path = "product_variants/create_form.html")]
// pub struct CreateFormPage {
//     pub products: Vec<ProductListDTO>,
//     pub brands: Vec<BrandsListDTO>,
//     pub form: FormDTO,
//     pub errors: Option<FormErrors>,
//     pub current_page: String,
//     pub error_message: Option<String>,
//     pub success_message: Option<String>,
// }

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
    pub error: Option<String>,
}

/// Partial HTML snippet template for HTMX/Dynamic live category search.
#[derive(Template, WebTemplate)]
#[template(path = "product_variants/search_results.html")]
pub struct VariantSearchResultsTemplate {
    pub variants: Vec<ProductVariantsResponseDTO>,
    pub query: String,
}

/// URL Query parameter extractor for category live-search requests.
#[derive(Debug, Deserialize)]
pub struct VariantSearchQuery {
    #[serde(default)]
    pub q: String,
}
