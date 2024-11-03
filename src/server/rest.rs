//! Common functionality for REST endpoints implementation

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::shared::validation::error::ValidationError;

#[derive(Debug)]
pub enum ErrorResponse {
    NotFound,
    ValidationFailed(Vec<ValidationError>),
    InternalError,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        match self {
            ErrorResponse::NotFound => StatusCode::NOT_FOUND.into_response(),
            ErrorResponse::ValidationFailed(errors) => {
                let message = ValidationMessage { errors };
                (StatusCode::BAD_REQUEST, Json(message)).into_response()
            }
            ErrorResponse::InternalError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationMessage {
    pub errors: Vec<ValidationError>,
}
