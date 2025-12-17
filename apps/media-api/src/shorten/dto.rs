use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateShortUrlRequest {
    pub url: String,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ShortUrlResponse {
    pub slug: String,
    pub short_url: String,
}
