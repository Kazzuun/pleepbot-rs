use envy::Error;
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pguser: String,
    pgpassword: String,
    pgdatabase: String,
    pghost: String,
    pgport: u16,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, Error> {
        dotenvy::dotenv().ok();
        envy::from_env()
    }

    fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.pguser, self.pgpassword, self.pghost, self.pgport, self.pgdatabase
        )
    }

    pub async fn pg_pool_connect(&self, max_connections: u32) -> sqlx::Result<PgPool> {
        PgPoolOptions::new()
            .max_connections(max_connections)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(&self.database_url())
            .await
    }
}
