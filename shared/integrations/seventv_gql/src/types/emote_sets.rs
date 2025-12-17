use chrono::{DateTime, Utc};
use cynic::{Enum, QueryFragment};
use ulid::Ulid;

use crate::schema;
use crate::types::emotes::Emote;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteSet {
    pub id: Ulid,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub capacity: Option<i32>,
    pub owner_id: Option<Ulid>,
    pub kind: EmoteSetKind,
    pub updated_at: DateTime<Utc>,
    // search_updated_at
    pub emotes: EmoteSetEmoteSearchResult,
    // owner
}

#[derive(Clone, Debug, PartialEq, Enum)]
pub enum EmoteSetKind {
    Normal,
    Personal,
    Global,
    Special,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteSetEmoteSearchResult {
    pub items: Vec<EmoteSetEmote>,
    pub total_count: i32,
    pub page_count: i32,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteSetEmote {
    pub id: Ulid,
    pub emote: Emote,
    pub alias: String,
    pub added_at: DateTime<Utc>,
    pub flags: EmoteSetEmoteFlags,
    pub added_by_id: Option<Ulid>,
    pub origin_set_id: Option<Ulid>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteSetEmoteFlags {
    pub zero_width: bool,
    pub override_conflicts: bool,
}
