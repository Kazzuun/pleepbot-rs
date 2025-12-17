use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;

use super::model::{LinkData, OriginalLink};

pub type SlugRepositoryState = Arc<dyn SlugRepository>;

#[async_trait]
pub trait SlugRepository: Send + Sync {
    async fn fetch_original_url(&self, slug: &str) -> Result<Option<OriginalLink>, sqlx::Error>;

    async fn add_click(&self, slug: &str) -> Result<(), sqlx::Error>;

    async fn fetch_link_data(&self, slug: &str) -> Result<Option<LinkData>, sqlx::Error>;
}

pub struct PgSlugRepository {
    db: PgPool,
}

impl PgSlugRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SlugRepository for PgSlugRepository {
    async fn fetch_original_url(&self, slug: &str) -> Result<Option<OriginalLink>, sqlx::Error> {
        let result = sqlx::query_as!(
            OriginalLink,
            "SELECT original_url, expires_at FROM links WHERE slug = $1",
            slug
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(result)
    }

    async fn add_click(&self, slug: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO clicks (link_id) VALUES ((SELECT id FROM links WHERE slug = $1))",
            slug
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    async fn fetch_link_data(&self, slug: &str) -> Result<Option<LinkData>, sqlx::Error> {
        let result = sqlx::query_as!(
            LinkData,
            r#"
            SELECT
                original_url,
                slug,
                created_at,
                expires_at,
                COALESCE((
                    SELECT COUNT(*)
                    FROM clicks
                    WHERE clicks.link_id = links.id
                ), 0)::INT AS "clicks!"
            FROM links
            WHERE slug = $1
            "#,
            slug
        )
        .fetch_optional(&self.db)
        .await?;

        Ok(result)
    }
}
