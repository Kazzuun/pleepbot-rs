use cynic::QueryFragment;

use super::vars::{UserByConnectionQueryVarsFields, UserVarsFields, UserSearchVarsFields};
use crate::schema;
use crate::types::users::{User, UserSearchResult};

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "Query", variables = "UserVars")]
pub struct UserQueryRoot {
    pub users: UserQuery,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "Query")]
pub struct UserSelfQueryRoot {
    pub users: UserSelfQuery,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "Query", variables = "UserByConnectionQueryVars")]
pub struct UserByConnectionRoot {
    pub users: UserByConnectionQuery,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "Query", variables = "UserSearchVars")]
pub struct UserSearchQueryRoot {
    pub users: UserSearchQuery,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "UserQuery", variables = "UserVars")]
pub struct UserQuery {
    #[arguments(id: $id)]
    pub user: Option<User>,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "UserQuery")]
pub struct UserSelfQuery {
    pub me: Option<User>,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "UserQuery", variables = "UserByConnectionQueryVars")]
pub struct UserByConnectionQuery {
    #[arguments(platform: $platform, platformId: $platform_id)]
    pub user_by_connection: Option<User>,
}

#[derive(Debug, QueryFragment)]
#[cynic(graphql_type = "UserQuery", variables = "UserSearchVars")]
pub struct UserSearchQuery {
    #[arguments(query: $query, page: $page, perPage: $per_page)]
    pub search: UserSearchResult,
}
