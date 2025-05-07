use std::io::Result; // TODO: use thiserror or anyhow?

use actix_web::{App, HttpServer, Responder, get, post, web};
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};


#[actix_web::main]
async fn main() -> Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(shorten)
            .service(redirect)
            .service(redirect_info)
    });

    let port = 8080;
    println!("Starting server on port {}", port);
    server.bind(("127.0.0.1", port))?.run().await
}

#[derive(Deserialize)]
struct ShortenPayload {
    url: String,
    expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
struct ShortenResponse {
    slug: String,
}

#[post("/shorten")]
async fn shorten(payload: web::Json<ShortenPayload>) -> Result<impl Responder> {
    let slug = nanoid!(6);
    let response = ShortenResponse { slug: slug.clone() };

    // TODO: store in database

    Ok(web::Json(response))
}

#[get("/{slug}")]
async fn redirect(path: web::Path<String>) -> Result<impl Responder> {
    let slug = path.into_inner();

    // TODO: retrieve the URL from the database using the slug and replace the placeholder URL

    Ok(web::Redirect::to("https://example.com"))
}

#[derive(Serialize)]
struct RedirectInfoResponse {
    slug: String,
    url: String,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    clicks: u32,
}

#[get("/{slug}/info")]
async fn redirect_info(path: web::Path<String>) -> Result<impl Responder> {
    let slug = path.into_inner();

    // TODO: retrieve data from the database using the slug

    let response = RedirectInfoResponse {
        slug: slug.clone(),
        url: "https://example.com".to_string(),
        created_at: Utc::now(),
        expires_at: None,
        clicks: 0,
    };
    Ok(web::Json(response))
}
