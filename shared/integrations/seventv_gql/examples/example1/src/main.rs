use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{fmt, prelude::*};
use ulid::Ulid;

use seventv_gql_api::SeventvGqlClient;

#[tokio::main]
async fn main() {
    let filter =
        EnvFilter::from_default_env().add_directive("seventv_gql_api=debug".parse().unwrap());

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();

    let client = SeventvGqlClient::new_authed("".to_string());

    let user_id = Ulid::from_string("01F46N0YZ80005W0EZC5BGNE80").unwrap();

    let user = client.users.get_user(user_id).await;
    println!("{:#?}", user)
}
