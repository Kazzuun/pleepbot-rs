use axum::{extract::Path, http::StatusCode};

pub async fn test(Path((channel, user)): Path<(String, String)>) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, "test\n".to_string())
}

pub async fn fallback() -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        "Erm this doesn't exist\n".to_string(),
    )
}
