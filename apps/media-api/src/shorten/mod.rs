mod dto;
pub mod repository;
mod service;

use axum::{Router, routing::post};

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/shorten", post(service::create_short_url))
}
