use askama::Template;
use askama_web::WebTemplate;
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

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariantsResponseDTO {
    pub id: i64,
    pub product_name: String,
    pub brand_name: String,
    pub name_ar: String,
    pub name_en: String,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
    pub is_active: bool,
    pub attr: sqlx::types::Json<serde_json::Value>, // or your specific Value type
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl ProductVariantsResponseDTO {
    /// يحوّل attr (JSON object) إلى أزواج (المفتاح، القيمة) للعرض في القالب
    pub fn attr_pairs(&self) -> Vec<(String, String)> {
        match &self.attr.0 {
            serde_json::Value::Object(map) => map
                .iter()
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    (k.clone(), val)
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

#[derive(Default, Debug, Serialize)]
pub struct FormErrors {
    pub product_name: Option<String>,
    pub brand_name: Option<String>,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub stock_quantity: Option<String>,
    pub reorder_threshold: Option<String>,
    pub is_active: Option<String>,
    pub notes: Option<String>,
    pub attr: Option<String>,
}

impl FormErrors {
    pub fn has_errors(&self) -> bool {
            self.name_ar.is_some()
            || self.name_en.is_some()
            || self.stock_quantity.is_some()
            || self.reorder_threshold.is_some()
            || self.is_active.is_some()
            || self.attr.is_some()
            || self.notes.is_some()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormDTO {
    pub product_name: String,
    pub brand_name: String,
    pub name_ar: String,
    pub name_en: String,
    pub stock_quantity: i32,
    pub reorder_threshold: i32,
    pub is_active: bool,
    pub attr: sqlx::types::Json<serde_json::Value>, // or your specific Value type
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl FormDTO {
    pub fn sanitize(&mut self) {
        self.product_name = self.product_name.trim().to_string();
        self.brand_name = self.brand_name.trim().to_string();
        self.name_ar = self.name_ar.trim().to_string();
        self.name_en = self.name_en.trim().to_string();
        self.notes = self
            .notes
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);
    }

   
}

impl FormDTO {
    pub fn attr_pairs(&self) -> Vec<(String, String)> {
        match &self.attr.0 {
            serde_json::Value::Object(map) => map
                .iter()
                .map(|(k, v)| {
                    let val = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    (k.clone(), val)
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

impl Default for FormDTO {
    fn default() -> Self {
        Self {
            product_name: String::new(),
            brand_name: String::new(),
            name_ar: String::new(),
            name_en: String::new(),
            stock_quantity: 0,
            reorder_threshold: 0,
            is_active: true,
            notes: None,
            attr: sqlx::types::Json(serde_json::json!({
                "color": "",
                "size": ""
            })),
        }
    }
}
// ############ Templates ##################
#[derive(Template, WebTemplate)]
#[template(path = "product_variants/index.html")]
pub struct ProductVariantsTemplate {
    pub product_variants: Vec<ProductVariantsResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,
}


#[derive(Template, WebTemplate)]
#[template(path = "product_variants/create_form.html")]
pub struct CreateFormPage {
    pub products: Vec<ProductListDTO>,
    pub brands: Vec<BrandsListDTO>,
    pub form: FormDTO,
    pub errors: Option<FormErrors>,
    pub current_page: String,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
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
