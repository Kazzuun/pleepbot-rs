use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use sqlx::PgPool;

use crate::error::AppError;

pub type ShortenRepositoryState = Arc<dyn ShortenRepository>;

#[async_trait]
pub trait ShortenRepository: Send + Sync {
    async fn create_short_url(
        &self,
        url: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<String, AppError>;
}

pub struct PgShortenRepository {
    db: PgPool,
}

impl PgShortenRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl ShortenRepository for PgShortenRepository {
    async fn create_short_url(
        &self,
        url: &str,
        expires_at: Option<DateTime<Utc>>,
    ) -> Result<String, AppError> {
        let slug = nanoid!(6);

        sqlx::query!(
            "INSERT INTO links(original_url, slug, expires_at) VALUES ($1, $2, $3)",
            url,
            slug,
            expires_at
        )
        .execute(&self.db)
        .await
        .map_err(|e| AppError::DatabaseError(e))?;

        Ok(slug)
    }
}
