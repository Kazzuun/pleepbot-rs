use serde::Serialize;
use utoipa::ToSchema;

use super::model::LinkData;

#[derive(Serialize, ToSchema)]
pub struct LinkDataResponse {
    original_url: String,
    slug: String,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    clicks: u32,
}

impl From<LinkData> for LinkDataResponse {
    fn from(value: LinkData) -> Self {
        Self {
            original_url: value.original_url,
            slug: value.slug,
            created_at: value.created_at,
            expires_at: value.expires_at,
            clicks: value.clicks.try_into().unwrap(),
        }
    }
}
