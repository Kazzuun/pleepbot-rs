use db::config::Config;

#[tokio::main]
async fn main() {
    Config::from_env()
        .pg_pool_connect(5)
        .await
        .expect("Failed to connect to the database");
}
