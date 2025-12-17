pub struct OriginalLink {
    pub original_url: String,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl OriginalLink {
    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |expiration_time| {
            expiration_time < chrono::Utc::now()
        })
    }
}

pub struct LinkData {
    pub original_url: String,
    pub slug: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub clicks: i32,
}

impl LinkData {
    pub fn is_expired(&self) -> bool {
        self.expires_at.map_or(false, |expiration_time| {
            expiration_time < chrono::Utc::now()
        })
    }
}
