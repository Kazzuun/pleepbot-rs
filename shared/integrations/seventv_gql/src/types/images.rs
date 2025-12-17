use cynic::QueryFragment;

use crate::schema;

#[derive(Clone, Debug, PartialEq, QueryFragment)]
pub struct Image {
    pub url: String,
    pub mime: String,
    pub size: i32,
    pub scale: i32,
    pub width: i32,
    pub height: i32,
    pub frame_count: i32,
}
