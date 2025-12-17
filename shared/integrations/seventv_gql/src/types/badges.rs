use chrono::{DateTime, Utc};
use cynic::QueryFragment;
use ulid::Ulid;

use crate::schema;
use crate::types::images::Image;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Badge {
    pub id: Ulid,
    pub name: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub images: Vec<Image>,
    pub created_by_id: Ulid,
    pub updated_at: DateTime<Utc>,
    // search_updated_at
}
