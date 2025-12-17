#![allow(dead_code)]

// Module required by cynic for auto-generation of the schema
#[cynic::schema("seventv")]
mod schema {}

cynic::impl_scalar!(ulid::Ulid, schema::Id);
cynic::impl_scalar!(chrono::DateTime<chrono::Utc>, schema::DateTime);

// Cynic query fragments and variables for queries and mutation
mod gql;
// Implementation of the client methods for the api
mod api;
// Public types
pub mod types;
// Public client
pub mod client;
// Internal http client
mod http_client;
// Global error types
pub mod error;

pub use client::SeventvGqlClient;
