use askama::Template;
use askama_web::WebTemplate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariantsResponseDTO {
    pub id: i64,
    pub product_name: String,
    pub brand_name: String,
    pub name_ar: String,
    pub name_en: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub shelf_location: Option<String>,
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
// ############ Templates ##################
#[derive(Template, WebTemplate)]
#[template(path = "product_variants/index.html")]
pub struct ProductVariantsTemplate {
    pub product_variants: Vec<ProductVariantsResponseDTO>,
   pub error_message: Option<String>,
   pub success_message: Option<String>,
   pub current_page: String,
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