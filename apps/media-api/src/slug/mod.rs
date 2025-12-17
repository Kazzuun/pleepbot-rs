mod dto;
mod model;
pub mod repository;
mod service;

use axum::{Router, routing::get};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{slug}", get(service::redirect_slug))
        .route("/{slug}/info", get(service::slug_info))
}
