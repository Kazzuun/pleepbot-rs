use chrono::{DateTime, Utc};
use cynic::{Enum, InlineFragments, QueryFragment};
use ulid::Ulid;

use crate::schema;
use crate::types::images::Image;
use crate::types::users::User;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Emote {
    pub id: Ulid,
    pub owner_id: Ulid,
    pub default_name: String,
    pub tags: Vec<String>,
    pub images_pending: bool,
    pub images: Vec<Image>,
    pub flags: EmoteFlags,
    /// Value between 0 and 3
    pub aspect_ratio: f64,
    pub attribution: Vec<EmoteAttribution>,
    pub scores: EmoteScores,
    pub deleted: bool,
    pub updated_at: DateTime<Utc>,
    // search_updated_at
    // owner
    // ranking
    // events
    // in_emote_sets
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteFlags {
    pub public_listed: bool,
    pub private: bool,
    pub nsfw: bool,
    pub default_zero_width: bool,
    pub approved_personal: bool,
    pub denied_personal: bool,
    pub animated: bool,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteAttribution {
    pub user_id: Ulid,
    pub added_at: DateTime<Utc>,
    // user
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteScores {
    pub trending_day: i32,
    pub trending_week: i32,
    pub trending_month: i32,
    pub top_daily: i32,
    pub top_weekly: i32,
    pub top_monthly: i32,
    pub top_all_time: i32,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteSearchResult {
    pub items: Vec<Emote>,
    pub total_count: i32,
    pub page_count: i32,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EmoteEvent {
    /// Id of the event
    pub id: Ulid,
    pub actor_id: Option<Ulid>,
    /// Id of the emote
    pub target_id: Ulid,
    pub data: EventEmoteData,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    // target
    // actor
}

#[derive(Clone, Debug, PartialEq, InlineFragments)]
pub enum EventEmoteData {
    EventEmoteDataUpload(EventEmoteDataUpload),
    EventEmoteDataProcess(EventEmoteDataProcess),
    EventEmoteDataChangeName(EventEmoteDataChangeName),
    EventEmoteDataMerge(EventEmoteDataMerge),
    EventEmoteDataChangeOwner(EventEmoteDataChangeOwner),
    EventEmoteDataChangeTags(EventEmoteDataChangeTags),
    EventEmoteDataChangeFlags(EventEmoteDataChangeFlags),
    EventEmoteDataDelete(EventEmoteDataDelete),

    #[cynic(fallback)]
    Other,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataUpload {
    /// Always false
    pub noop: bool,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataProcess {
    pub event: ImageProcessorEvent,
}

#[derive(Clone, Debug, PartialEq, Enum)]
pub enum ImageProcessorEvent {
    Success,
    Fail,
    Cancel,
    Start,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataChangeName {
    pub old_name: String,
    pub new_name: String,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataMerge {
    pub new_emote_id: Ulid,
    pub new_emote: Emote,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataChangeOwner {
    pub old_owner_id: Ulid,
    pub new_owner_id: Ulid,
    pub old_owner: Option<User>,
    pub new_owner: Option<User>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataChangeTags {
    pub old_tags: Vec<String>,
    pub new_tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataChangeFlags {
    pub old_flags: EmoteFlags,
    pub new_flags: EmoteFlags,
}

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct EventEmoteDataDelete {
    /// Always false
    pub noop: bool,
}
