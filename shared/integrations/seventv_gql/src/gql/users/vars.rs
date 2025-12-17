use cynic::QueryVariables;
use ulid::Ulid;

use crate::schema;
use crate::types::users::Platform;

#[derive(Debug, QueryVariables)]
pub struct UserVars {
    pub id: Ulid,
}

#[derive(Debug, QueryVariables)]
pub struct UserByConnectionQueryVars {
    pub platform: Platform,
    pub platform_id: String,
}

#[derive(Debug, QueryVariables)]
pub struct UserSearchVars {
    pub query: String,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}
