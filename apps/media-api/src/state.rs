use std::sync::Arc;

use axum::extract::FromRef;
use db::connection::DatabaseConfig;

use crate::shorten::repository::{PgShortenRepository, ShortenRepository};
use crate::slug::repository::{PgSlugRepository, SlugRepository};

#[derive(FromRef, Clone)]
pub struct AppState {
    shorten_repository: Arc<dyn ShortenRepository>,
    slug_repository: Arc<dyn SlugRepository>,
}

impl AppState {
    pub async fn new() -> Result<Self, sqlx::Error> {
        // TODO: connect to the database
        let db_config = DatabaseConfig::from_env().expect("Failed to load database settings");
        let db = db_config
            .pg_pool_connect(5)
            .await
            .expect("Failed to connect to the database");

        let shorten_repository: Arc<dyn ShortenRepository> =
            Arc::new(PgShortenRepository::new(db.clone()));

        let slug_repository: Arc<dyn SlugRepository> = Arc::new(PgSlugRepository::new(db.clone()));

        Ok(Self {
            shorten_repository,
            slug_repository,
        })
    }
}
