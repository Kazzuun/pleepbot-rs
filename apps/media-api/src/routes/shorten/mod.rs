mod handler;
mod service;

use axum::{Router, routing::post};
use sqlx::postgres::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new().route("/shorten", post(handler::create_short_url))
}
