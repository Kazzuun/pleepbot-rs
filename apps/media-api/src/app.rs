use axum::Router;

use crate::shorten;
use crate::slug;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(shorten::router())
        .merge(slug::router())
        .with_state(state)
}
