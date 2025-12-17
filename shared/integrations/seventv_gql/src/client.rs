use std::collections::HashMap;

use crate::api::{
    BadgeService, EmoteService, EmoteSetService, PaintService, ProductService, RoleService,
    StoreService, UserEditorService, UserService,
};
use crate::http_client::{Authenticated, ReqwestHttpClient, Unauthenticated};

#[derive(Clone)]
pub struct SeventvGqlClient<C> {
    // TODO: Should they be wrapped in Arc?
    pub badges: BadgeService<C>,
    pub emote_sets: EmoteSetService<C>,
    pub emotes: EmoteService<C>,
    pub paints: PaintService<C>,
    pub products: ProductService<C>,
    pub roles: RoleService<C>,
    pub store: StoreService<C>,
    pub user_editors: UserEditorService<C>,
    pub users: UserService<C>,
}

impl SeventvGqlClient<ReqwestHttpClient<Unauthenticated>> {
    pub fn new() -> Self {
        let http_client = ReqwestHttpClient::new();

        // All services are created even when all the methods of the service need authentication.
        // In that case, the service just doesn't have any methods but is still discoverable.
        Self {
            badges: BadgeService::new(http_client.clone()),
            emote_sets: EmoteSetService::new(http_client.clone()),
            emotes: EmoteService::new(http_client.clone()),
            paints: PaintService::new(http_client.clone()),
            products: ProductService::new(http_client.clone()),
            roles: RoleService::new(http_client.clone()),
            store: StoreService::new(http_client.clone()),
            user_editors: UserEditorService::new(http_client.clone()),
            users: UserService::new(http_client.clone()),
        }
    }
}

impl SeventvGqlClient<ReqwestHttpClient<Authenticated>> {
    pub fn new_authed(token: String) -> Self {
        let http_client = ReqwestHttpClient::new_authed(token);

        // All the services and their methods are available, including ones that don't need authentication
        Self {
            badges: BadgeService::new(http_client.clone()),
            emote_sets: EmoteSetService::new(http_client.clone()),
            emotes: EmoteService::new(http_client.clone()),
            paints: PaintService::new(http_client.clone()),
            products: ProductService::new(http_client.clone()),
            roles: RoleService::new(http_client.clone()),
            store: StoreService::new(http_client.clone()),
            user_editors: UserEditorService::new(http_client.clone()),
            users: UserService::new(http_client.clone()),
        }
    }
}

// pub struct SeventvGqlClientBuilder {
//     headers: HashMap<String, String>,
//     endpoint: String,
//     token: Option<String>,
// }

// impl Default for SeventvGqlClientBuilder {
//     fn default() -> Self {
//         SeventvGqlClientBuilder {
//             headers: HashMap::new(),
//             endpoint: "https://api.7tv.app/v4/gql".to_string(),
//             token: None,
//         }
//     }
// }

// impl SeventvGqlClientBuilder {
//     pub fn with_headers(&mut self, headers: HashMap<String, String>) -> &Self {
//         for (key, value) in headers.iter() {
//             self.headers.insert(key.clone(), value.clone());
//         }
//         self
//     }

//     pub fn with_endpoint(&mut self) -> &Self {
//         self
//     }

//     pub fn with_token(&mut self, token: String) -> &Self {
//         self.token = Some(token.clone());
//         self.headers.insert("Authorization".to_string(), format!("Bearer {}", token));
//         self
//     }

//     pub fn build(&self) {
//         let http_client: ReqwestHttpClient<_> = if let Some(token) = self.token {
//             ReqwestHttpClient::new_authed(token)
//         } else {
//             ReqwestHttpClient::new()
//         };

//         // All the services and their methods are available, including ones that don't need authentication
//         SeventvGqlClient {
//             badges: BadgeService::new(http_client.clone()),
//             emote_sets: EmoteSetService::new(http_client.clone()),
//             emotes: EmoteService::new(http_client.clone()),
//             paints: PaintService::new(http_client.clone()),
//             products: ProductService::new(http_client.clone()),
//             roles: RoleService::new(http_client.clone()),
//             store: StoreService::new(http_client.clone()),
//             user_editors: UserEditorService::new(http_client.clone()),
//             users: UserService::new(http_client.clone()),
//         }
//     }
// }

// impl<T> SeventvGqlClient<ReqwestHttpClient<T>> {
//     pub fn builder() -> SeventvGqlClientBuilder {

//     }
// }
