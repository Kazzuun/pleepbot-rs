mod app;
mod error;
mod routes;

use db::connection::DatabaseConfig;

// https://github.com/tokio-rs/axum/tree/main/examples/sqlx-postgres
// https://github.com/nakamuraos/axum-postgres-boilerplate

#[tokio::main]
async fn main() {
    let db_config = DatabaseConfig::from_env().expect("Failed to load database settings");
    let pool = db_config
        .pg_pool_connect(5)
        .await
        .expect("Failed to connect to the database");

    let app = app::router(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
