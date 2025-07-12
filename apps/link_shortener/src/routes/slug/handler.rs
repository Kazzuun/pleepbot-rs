use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::Response,
};
use sqlx::postgres::PgPool;

use crate::{
    error::AppError,
    routes::slug::service::{self, LinkData},
};

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
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let original_url = service::fetch_original_url(&pool, &slug).await?;

    match original_url {
        None => Err(AppError::LinkNotFound),
        Some(url) => {
            service::add_click(&pool, &slug).await?;

            let response = Response::builder()
                .status(StatusCode::MOVED_PERMANENTLY)
                .header(header::LOCATION, &url)
                .body(Body::empty())
                .unwrap();

            Ok(response)
        }
    }
}

#[rustfmt::skip]
#[utoipa::path(
    get, 
    path = "/{slug}/info", 
    params(
        ("slug" = String, Path, description = "Slug for the shortened URL")
    ),
    responses(
        (status = 200, description = "Data related to the shortened URL", body = LinkData),
        (status = 404, description = "Slug not found"),
    )
)]
pub async fn slug_info(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<Json<LinkData>, AppError> {
    let link_data = service::fetch_link_data(&pool, &slug).await?;

    match link_data {
        None => Err(AppError::LinkNotFound),
        Some(link_data) => Ok(Json(link_data)),
    }
}
