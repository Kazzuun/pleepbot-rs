use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::Deserialize;
use ulid::Ulid;

#[derive(Debug, Deserialize)]
struct EmoteData {}

#[derive(Debug, Deserialize)]
struct Emote {
    id: Ulid,
    name: String,
    flags: i32,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    timestamp: DateTime<Utc>,
    actor_id: Ulid,
    data: EmoteData,
}

#[derive(Debug, Deserialize)]
struct EmoteSet {
    id: Ulid,
    name: String,
    flags: i32, // TODO
    tags: Vec<String>,
    immutable: bool,
    privileged: bool,
    emotes: Vec<Emote>,
    emote_count: i32,
    capacity: i32,
}

#[derive(Debug, Deserialize)]
struct User {}

#[derive(Debug, Deserialize)]
struct UserData {
    id: String,
    platform: String,
    username: String,
    display_name: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    linked_at: DateTime<Utc>,
    emote_capacity: i32,
    emote_set_id: Ulid,
    emote_set: EmoteSet,
    user: User,
}


// TODO: need a function that returns all the ids of the emotes present in the message
// do this with a has map; id -> count, or a vec of tuples?

async fn fetch_seventv_user_data(twitch_id: &str) -> Result<UserData, reqwest::Error> {
    let url = format!("https://7tv.io/v3/users/twitch/{}", twitch_id);
    let client = Client::new();
    let response = client
        .get(url)
        .send()
        .await
        .expect("meow")
        .json::<UserData>()
        .await;
    response
}
