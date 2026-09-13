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


#[derive(Deserialize, Serialize, FromRow, Clone, Debug)]
pub struct CategoryResponseDTO {
    pub id: i64,
    pub name_en: String,
    pub name_ar: String,
    pub parent_id: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryTree {
    pub main: CategoryResponseDTO,
    pub subs: Vec<CategoryResponseDTO>,
}

impl CategoryTree {
    pub fn build_tree(categories: Vec<CategoryResponseDTO>) -> Vec<Self> {
        let (mains, subs): (Vec<_>, Vec<_>) =
            categories.into_iter().partition(|c| c.parent_id.is_none());

        mains
            .into_iter()
            .map(|main_cat| {
                let child_subs = subs
                    .iter()
                    .filter(|s| s.parent_id == Some(main_cat.id))
                    .cloned()
                    .collect();

                CategoryTree {
                    main: main_cat,
                    subs: child_subs,
                }
            })
            .collect()
    }
}


// ############ Templates ##################

#[derive(Template, WebTemplate)]
#[template(path = "categories/index.html")]
pub struct CategoryTemplate {
    pub category_tree: Vec<CategoryTree>,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
    pub current_page: String,
}