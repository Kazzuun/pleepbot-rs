use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{TypedHeader, headers::Host};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use url::Url;

use crate::{error::AppError, routes::shorten::service};

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateShortUrlRequest {
    url: String,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ShortUrlResponse {
    slug: String,
    short_url: String,
}

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
    State(pool): State<PgPool>,
    Json(payload): Json<CreateShortUrlRequest>,
) -> Result<(StatusCode, Json<ShortUrlResponse>), AppError> {
    // TODO: expires after a certain number of clicks
    let url = if payload.url.starts_with("http://") || payload.url.starts_with("https://") {
        Url::parse(&payload.url)
    } else {
        let fixed = format!("https://{}", payload.url);
        Url::parse(&fixed)
    }
    .map_err(|_| AppError::InvalidRequest("Invalid URL".to_string()))?;

    if let Some(expiration_time) = payload.expires_at {
        if expiration_time < chrono::Utc::now() {
            return Err(AppError::InvalidRequest(
                "Expiration time cannot be in the past".to_string(),
            ));
        }
    }

    let slug = service::create_short_url(&pool, url.as_str(), payload.expires_at).await?;

    let url = if host.hostname() == "localhost" {
        // Only used for testing locally
        format!(
            "http://{}:{}/{}",
            host.hostname(),
            host.port().ok_or(AppError::InvalidRequest(
                "Port not specified for localhost in Host header".to_string()
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
