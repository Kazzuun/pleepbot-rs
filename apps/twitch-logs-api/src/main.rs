use std::sync::Arc;

use external::twitch::TwitchMessageListener;
use tokio::sync::Mutex;

mod external;
mod http;

// Take inspiration from https://github.com/saliven/twitch-logger

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let twitch_chat = Arc::new(Mutex::new(TwitchMessageListener::init().await));

    let message_listener_handle = {
        let twitch_chat_clone = Arc::clone(&twitch_chat);
        tokio::spawn(async move { twitch_chat_clone.lock().await.connect().await })
    };
    let http_handle = tokio::spawn(http::run());
    let exit_handle = tokio::spawn(tokio::signal::ctrl_c());

    tokio::select! {
        r = message_listener_handle => tracing::warn!("Twitch IRC message listener exited: {:?}", r),
        r = http_handle => tracing::warn!("Api exited: {:?}", r),
        _ = exit_handle => tracing::warn!("Ctrl+c received"),
    }
}
