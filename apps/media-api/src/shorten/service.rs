use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{TypedHeader, headers::Host};
use url::Url;

use super::dto::{CreateShortUrlRequest, ShortUrlResponse};
use super::repository::ShortenRepositoryState;
use crate::error::AppError;

#[rustfmt::skip]
#[utoipa::path(
    post,
    path = "/shorten",
    request_body = CreateShortUrlRequest,
    responses(
        (status = 201, description = "Short URL created", body = ShortUrlResponse),
        (status = 400, description = "Invalid URL"),
        (status = 400, description = "Invalid expiration date")
    )
)]
pub async fn create_short_url(
    TypedHeader(host): TypedHeader<Host>,
    State(shorten_repo): State<ShortenRepositoryState>,
    Json(payload): Json<CreateShortUrlRequest>,
) -> Result<(StatusCode, Json<ShortUrlResponse>), AppError> {
    // TODO: expires after a certain number of clicks
    let url = if payload.url.starts_with("http://") || payload.url.starts_with("https://") {
        Url::parse(&payload.url)
    } else {
        let fixed = format!("https://{}", payload.url);
        Url::parse(&fixed)
    }
    .map_err(|_| AppError::InvalidRequest("invalid URL".to_string()))?;

    if let Some(expiration_time) = payload.expires_at {
        if expiration_time < chrono::Utc::now() {
            return Err(AppError::InvalidRequest(
                "expiration time cannot be in the past".to_string(),
            ));
        }
    }

    let slug = shorten_repo
        .create_short_url(url.as_str(), payload.expires_at)
        .await?;

    let url = if host.hostname() == "localhost" {
        // Only used for testing locally
        format!(
            "http://{}:{}/{}",
            host.hostname(),
            host.port().ok_or(AppError::InvalidRequest(
                "port not specified for localhost in Host header".to_string()
            ))?,
            slug
        )
    } else {
        // Assume https
        format!("https://{}/{}", host.hostname(), slug)
    };

    let response = ShortUrlResponse {
        short_url: url,
        slug,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
