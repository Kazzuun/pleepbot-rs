use cynic::{GraphQlError, http::CynicReqwestError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("GraphQL errors: {0:?}")]
    GQLError(Vec<GraphQlError>),

    #[error("Missing data in GraphQL response")]
    MissingData,

    #[error("Error making HTTP request: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Server returned {0}: {1}")]
    HttpError(reqwest::StatusCode, String),
}

impl From<CynicReqwestError> for ApiError {
    fn from(err: CynicReqwestError) -> Self {
        match err {
            CynicReqwestError::ReqwestError(inner) => ApiError::ReqwestError(inner),
            CynicReqwestError::ErrorResponse(status, body) => ApiError::HttpError(status, body),
        }
    }
}
