use axum::{routing::get, Router};

pub mod handler;

// https://github.com/SevenTV/SevenTV/blob/main/apps/api/src/http/v3/mod.rs
pub fn routes() -> Router {
    Router::new()
        .route("/channels", get(handler::test).post(handler::test))
        .route("/emotes/:channel", get(handler::test))
        .route("/logs/:channel/:user", get(handler::test))
        .fallback(handler::fallback)
}
