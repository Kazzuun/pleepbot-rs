mod app;
mod error;
mod routes;

use std::net::{Ipv4Addr, SocketAddr};

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

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
