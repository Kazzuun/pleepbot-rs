use serde::Serialize;
use sqlx::PgPool;

use crate::error::AppError;

#[derive(Serialize, utoipa::ToSchema)]
pub struct LinkData {
    original_url: String,
    slug: String,
    created_at: chrono::DateTime<chrono::Utc>,
    expires_at: Option<chrono::DateTime<chrono::Utc>>,
    clicks: u32,
}

pub async fn fetch_original_url(pool: &PgPool, slug: &str) -> Result<Option<String>, AppError> {
    let result = sqlx::query!(
        "SELECT original_url, expires_at FROM links WHERE slug = $1",
        slug
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    match result {
        None => Ok(None),
        Some(row) => {
            if let Some(expires_at) = row.expires_at {
                if expires_at < chrono::Utc::now() {
                    return Err(AppError::LinkExpired);
                }
            }

            Ok(Some(row.original_url))
        }
    }
}

pub async fn add_click(pool: &PgPool, slug: &str) -> Result<(), AppError> {
    sqlx::query!(
        "INSERT INTO clicks (link_id) VALUES ((SELECT id FROM links WHERE slug = $1))",
        slug
    )
    .execute(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    Ok(())
}

pub async fn fetch_link_data(pool: &PgPool, slug: &str) -> Result<Option<LinkData>, AppError> {
    let result = sqlx::query!(
        "SELECT 
            original_url, 
            slug, 
            created_at, 
            expires_at, 
            (
                SELECT COUNT(*) 
                FROM clicks 
                WHERE clicks.link_id = links.id
            ) AS clicks
        FROM links
        WHERE slug = $1",
        slug
    )
    .fetch_optional(pool)
    .await
    .map_err(AppError::DatabaseError)?;

    // Manually unwrap the Record type to convert clicks into u32
    let data = result.map(|r| LinkData {
        original_url: r.original_url,
        slug: r.slug,
        created_at: r.created_at,
        expires_at: r.expires_at,
        clicks: r.clicks.unwrap_or(0).try_into().unwrap(),
    });

    Ok(data)
}
