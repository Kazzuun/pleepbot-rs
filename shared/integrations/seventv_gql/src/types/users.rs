use chrono::{DateTime, Utc};
use cynic::{Enum, QueryFragment};
use ulid::Ulid;

use crate::schema;
use crate::types::entitlements::EntitlementEdgeAnyBadge;
use crate::types::images::Image;
use crate::types::paints::Color;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct User {
    pub id: Ulid,
    pub connections: Vec<UserConnection>,
    pub updated_at: DateTime<Utc>,
    // TODO: documentation on different role ranks
    pub highest_role_rank: i32,
    pub highest_role_color: Option<Color>,
    pub role_ids: Vec<Ulid>,
    pub main_connection: Option<UserConnection>,
    // TODO: add emote set fields?
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct UserConnection {
    pub platform: Platform,
    pub platform_id: String,
    pub platform_username: String,
    pub platform_display_name: String,
    pub platform_avatar_url: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub linked_at: DateTime<Utc>,
    pub allow_login: bool,
}

#[derive(Clone, Debug, PartialEq, Enum)]
pub enum Platform {
    Twitch,
    Discord,
    Google,
    Kick,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct UserSearchResult {
    pub items: Vec<User>,
    pub total_count: i32,
    pub page_count: i32,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct UserStyle {
    active_badge_id: Option<Ulid>,
    active_paint_id: Option<Ulid>,
    active_emote_set_id: Option<Ulid>,
    active_profile_picture_id: Option<Ulid>,
    active_profile_picture: Option<UserProfilePicture>,
    pending_profile_picture_id: Option<Ulid>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct UserProfilePicture {
    pub id: Ulid,
    pub user_id: Ulid,
    pub images: Vec<Image>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct UserInventory {
    pub badges: Vec<EntitlementEdgeAnyBadge>,
}
