use askama::Template;
use askama_web::WebTemplate;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::prelude::FromRow;

pub fn empty_string_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|s| !s.trim().is_empty()))
}

//######################################################
//######################################################
#[derive(Debug, Serialize, FromRow, Clone, Deserialize)]
pub struct BrandResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
//######################################################
//######################################################

#[derive(Default, Debug, Serialize)]
pub struct FormErrors {
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub notes: Option<String>,
}

impl FormErrors {
    pub fn has_errors(&self) -> bool {
        self.name_ar.is_some() || self.name_en.is_some() || self.notes.is_some()
    }
}

#[derive(Debug, Deserialize, Default, Serialize)]
pub struct FormDTO {
    pub name_ar: String,
    pub name_en: String,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub notes: Option<String>,
}

impl FormDTO {
    pub fn sanitize(&mut self) {
        self.name_ar = self.name_ar.trim().to_string();
        self.name_en = self.name_en.trim().to_string();
        self.notes = self.notes.take().map(|s| s.trim().to_string());
    }

    pub fn validate(&self) -> Result<(), FormErrors> {
    let mut errors = FormErrors::default();

    let name_ar = self.name_ar.trim();
    if name_ar.is_empty() {
        errors.name_ar = Some("لا يمكن ترك الاسم خالياً".to_string());
    } else if name_ar.chars().count() > 100 {
        errors.name_ar = Some("لا يمكن أن تتعدى خانة الاسم 100 حرف".to_string());
    }

    let name_en = self.name_en.trim();
    if name_en.is_empty() {
        errors.name_en = Some("This field is not allowed to be empty".to_string());
    } else if name_en.chars().count() > 100 {
        errors.name_en = Some("This field is not allowed to be more than 100 characters".to_string());
    }

    if let Some(notes) = self.notes.as_deref() {
        if notes.trim().chars().count() > 500 {
            errors.notes = Some("لا يمكن أن يتعدى هذا الحقل 500 حرف".to_string());
        }
    }

    if errors.has_errors() {
        Err(errors)
    } else {
        Ok(())
    }
}
}

// ============================================================================
// SECTION 4: ASKAMA TEMPLATES & UI FILTERS
// ============================================================================

/// Main page Askama template rendering brand management dashboard (`brands.html`).
#[derive(Template, WebTemplate)]
#[template(path = "brands/index.html")]
pub struct BrandsTemplate {
    pub brands: Vec<BrandResponseDTO>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,
    
}