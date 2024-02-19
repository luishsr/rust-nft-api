use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// Define a custom application error type using `thiserror`
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Web3 error: {0}")]
    Web3Error(#[from] web3::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    GenericError(String),

    #[error("Smart contract error: {0}")]
    NotFound(String),
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::GenericError(format!("An error occurred: {}", err))
    }
}

// Implement `IntoResponse` for `AppError` to convert it into an HTTP response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message.clone()),
            AppError::InternalServerError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.clone())
            }
            AppError::Web3Error(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            AppError::SerdeError(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message.to_string())
            }
            AppError::GenericError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message.clone()),
            AppError::NotFound(message) => (StatusCode::INTERNAL_SERVER_ERROR, message.clone()),
        };

        let body = Json(json!({ "error": error_message })).into_response();
        (status, body).into_response()
    }
}

// Custom UploadError type for file upload errors
#[derive(Error, Debug)]
pub enum UploadError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum SignatureError {
    #[error("Hex decoding error: {0}")]
    HexDecodeError(#[from] hex::FromHexError),
}

impl From<SignatureError> for AppError {
    fn from(err: SignatureError) -> AppError {
        match err {
            SignatureError::HexDecodeError(_) => {
                AppError::BadRequest("Invalid hex format".to_string())
            }
        }
    }
}

// Implement `IntoResponse` for `UploadError` to convert it into an HTTP response
impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            UploadError::IoError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        let body = Json(json!({ "error": error_message })).into_response();
        (status, body).into_response()
    }
}
