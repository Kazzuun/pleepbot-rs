mod handler;
mod service;

use axum::{Router, routing::get};
use sqlx::postgres::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/{slug}", get(handler::redirect_slug))
        .route("/{slug}/info", get(handler::slug_info))
}
