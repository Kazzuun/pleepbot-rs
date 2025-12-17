use chrono::{DateTime, Utc};
use cynic::QueryFragment;
use ulid::Ulid;

use crate::schema;
use crate::types::paints::Color;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Role {
    pub id: Ulid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created_by_id: Ulid,
    pub color: Option<Color>,
    pub updated_at: DateTime<Utc>,
    // search_updated_at
    // created_by
}
