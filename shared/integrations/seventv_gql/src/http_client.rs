use std::fmt::Debug;

use cynic::http::ReqwestExt;
use cynic::serde::Serialize;
use cynic::serde::de::DeserializeOwned;
use cynic::{GraphQlResponse, Operation};
use tracing::{debug, instrument};

use crate::error::ApiError;

const ENDPOINT: &str = "https://api.7tv.app/v4/gql";

pub trait HttpClient {
    fn make_request<Q, V>(
        &self,
        op: Operation<Q, V>,
    ) -> impl std::future::Future<Output = Result<Q, ApiError>> + Send
    where
        Q: DeserializeOwned + Debug + 'static,
        V: Serialize + Send;
}

pub trait HttpClientAuthed {
    fn make_request_authed<Q, V>(
        &self,
        op: Operation<Q, V>,
    ) -> impl std::future::Future<Output = Result<Q, ApiError>> + Send
    where
        Q: DeserializeOwned + Debug + 'static,
        V: Serialize + Send;
}

#[derive(Clone)]
pub struct Unauthenticated;

#[derive(Clone)]
pub struct Authenticated {
    pub token: String,
}

#[derive(Clone)]
pub struct ReqwestHttpClient<A> {
    client: reqwest::Client,
    auth: A,
}

impl ReqwestHttpClient<Unauthenticated> {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            auth: Unauthenticated,
        }
    }
}

impl ReqwestHttpClient<Authenticated> {
    pub fn new_authed(token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            auth: Authenticated { token },
        }
    }
}

impl<A> ReqwestHttpClient<A> {
    fn extract_data<Q>(&self, resp: GraphQlResponse<Q>) -> Result<Q, ApiError> {
        match (resp.data, resp.errors) {
            // Data without errors
            (Some(data), None) => Ok(data),
            // Errors without data
            (None, Some(errs)) => Err(ApiError::GQLError(errs)),
            // These two states shouldn't happen
            (Some(_), Some(errs)) => Err(ApiError::GQLError(errs)),
            (None, None) => Err(ApiError::MissingData),
        }
    }
}

impl<A: Sync> HttpClient for ReqwestHttpClient<A> {
    #[instrument(skip(self, op), level = "debug")]
    async fn make_request<Q, V>(&self, op: Operation<Q, V>) -> Result<Q, ApiError>
    where
        Q: DeserializeOwned + Debug + 'static,
        V: Serialize + Send,
    {
        debug!(
            query = ?op.query,
            "Sending a GQL query without authentication"
        );
        let resp = self.client.post(ENDPOINT).run_graphql(op).await?;
        debug!(response = ?resp, "Got a response from the API");
        self.extract_data(resp)
    }
}

impl HttpClientAuthed for ReqwestHttpClient<Authenticated> {
    #[instrument(skip(self, op), level = "debug")]
    async fn make_request_authed<Q, V>(&self, op: Operation<Q, V>) -> Result<Q, ApiError>
    where
        Q: DeserializeOwned + Debug + 'static,
        V: Serialize + Send,
    {
        debug!(
            query = ?op.query,
            "Sending a GQL query with authentication"
        );
        let resp = self
            .client
            .post(ENDPOINT)
            .header("Authorization", format!("Bearer {}", self.auth.token))
            .run_graphql(op)
            .await?;
        debug!(response = ?resp, "Got a response from the API");
        self.extract_data(resp)
    }
}
