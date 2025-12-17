use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::Response,
};

use super::dto::LinkDataResponse;
use super::repository::SlugRepositoryState;
use crate::error::AppError;

#[rustfmt::skip]
#[utoipa::path(
    get,
    path = "/{slug}",
    params(
        ("slug" = String, Path, description = "Slug for the shortened URL")
    ),
    responses(
        (status = 301, description = "Permanent redirect"),
        (status = 404, description = "Slug not found"),
        (status = 410, description = "Link expired")
    )
)]
pub async fn redirect_slug(
    State(slug_repo): State<SlugRepositoryState>,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let original_link = slug_repo.fetch_original_url(&slug).await?;

    let original_link = original_link.ok_or(AppError::LinkNotFound)?;
    if original_link.is_expired() {
        return Err(AppError::LinkExpired)
    }

    slug_repo.add_click(&slug).await?;

    let response = Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header(header::LOCATION, &original_link.original_url)
        .body(Body::empty())
        .map_err(|_| AppError::InternalError("building response failed".to_string()))?;

    Ok(response)
}

#[rustfmt::skip]
#[utoipa::path(
    get,
    path = "/{slug}/info",
    params(
        ("slug" = String, Path, description = "Slug for the shortened URL")
    ),
    responses(
        (status = 200, description = "Data related to the shortened URL", body = LinkDataResponse),
        (status = 404, description = "Slug not found"),
    )
)]
pub async fn slug_info(
    State(slug_repo): State<SlugRepositoryState>,
    Path(slug): Path<String>,
) -> Result<Json<LinkDataResponse>, AppError> {
    let link_data = slug_repo.fetch_link_data(&slug).await?;

    match link_data {
        None => Err(AppError::LinkNotFound),
        Some(link_data) => Ok(Json(link_data.into())),
    }
}
