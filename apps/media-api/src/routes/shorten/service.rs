use chrono::{DateTime, Utc};
use nanoid::nanoid;
use sqlx::PgPool;

use crate::error::AppError;

pub async fn create_short_url(
    pool: &PgPool,
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
    .execute(pool)
    .await
    .map_err(|e| AppError::DatabaseError(e))?;

    Ok(slug)
}
