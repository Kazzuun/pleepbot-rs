mod app;
mod error;
mod shorten;
mod slug;
mod state;

use std::net::{Ipv4Addr, SocketAddr};

use crate::state::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::new()
        .await
        .expect("Failed to initialize app state");

    let app = app::router(state);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve app");
}
