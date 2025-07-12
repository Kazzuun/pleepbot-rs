use axum::Router;
use sqlx::PgPool;

use crate::routes::shorten;
use crate::routes::slug;

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .merge(shorten::router())
        .merge(slug::router())
        .with_state(pool)
}
