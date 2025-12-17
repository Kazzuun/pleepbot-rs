use cynic::QueryBuilder;
use ulid::Ulid;

use crate::error::ApiError;
use crate::gql::users::query::{
    UserByConnectionRoot, UserQueryRoot, UserSearchQueryRoot, UserSelfQueryRoot,
};
use crate::gql::users::vars::{UserByConnectionQueryVars, UserSearchVars, UserVars};
use crate::http_client::{HttpClient, HttpClientAuthed};
use crate::types::badges::Badge;
use crate::types::paints::Paint;
use crate::types::users::{Platform, User, UserSearchResult};

#[derive(Clone)]
pub struct UserService<C> {
    client: C,
}

impl<C: HttpClient> UserService<C> {
    pub(crate) fn new(client: C) -> Self {
        Self { client }
    }

    pub async fn get_user(&self, user_id: Ulid) -> Result<Option<User>, ApiError> {
        let vars = UserVars { id: user_id };
        let op = UserQueryRoot::build(vars);
        let resp = self.client.make_request(op).await?;
        Ok(resp.users.user)
    }

    pub async fn get_user_by_connection(
        &self,
        platform: Platform,
        platform_id: String,
    ) -> Result<Option<User>, ApiError> {
        let vars = UserByConnectionQueryVars {
            platform,
            platform_id,
        };
        let op = UserByConnectionRoot::build(vars);
        let resp = self.client.make_request(op).await?;
        Ok(resp.users.user_by_connection)
    }

    pub async fn search(
        &self,
        query: String,
        page: Option<i32>,
        per_page: Option<i32>,
    ) -> Result<UserSearchResult, ApiError> {
        let vars = UserSearchVars {
            query,
            page,
            per_page,
        };
        let op = UserSearchQueryRoot::build(vars);
        let resp = self.client.make_request(op).await?;
        Ok(resp.users.search)
    }

    pub async fn get_owned_emotes(&self, user_id: Ulid) {}

    pub async fn get_owned_emote_sets(&self, user_id: Ulid) {}

    pub async fn get_events(&self, user_id: Ulid) {}

    // There exists a badge that is null but has an id. This badge is filtered out.
    pub async fn get_badges(&self, user_id: Ulid) -> Result<Vec<Badge>, ApiError> {
        // data.map(|inv| {
        //     inv.badges
        //         .iter()
        //         .filter_map(|edge| edge.to.badge.clone())
        //         .collect()
        // })

        todo!();
    }

    pub async fn get_paints(&self, user_id: Ulid) -> Result<Vec<Paint>, ApiError> {
        todo!();
    }
}

impl<C: HttpClientAuthed> UserService<C> {
    /// Returns null when used without a valid token
    pub async fn get_self(&self) -> Result<Option<User>, ApiError> {
        let op = UserSelfQueryRoot::build(());
        let resp = self.client.make_request_authed(op).await?;
        Ok(resp.users.me)
    }
}
