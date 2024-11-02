use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::inventory::app::service::{InventoryError, InventoryService};
use crate::inventory::domain::name::Name;
use crate::inventory::domain::part::{Part, PartId};

pub async fn register_part(
    State(inventory): State<Arc<dyn InventoryService>>,
    Json(command): Json<RegisterPartCommand>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let name = match Name::try_from(command.name) {
        Ok(name) => name,
        Err(validation_error) => {
            return Err(validation_error.into_response());
        }
    };
    let result = inventory.register_part(name).await;
    match result {
        Ok(part_id) => {
            let response = RegisterPartResponse { id: part_id };
            Ok(response)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    }
}

pub async fn view_part(
    State(inventory): State<Arc<dyn InventoryService>>,
    Path(part_id): Path<Uuid>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let result = inventory.view_part(part_id).await;
    match result {
        Ok(part) => {
            let part_view = PartView::from(part);
            Ok(part_view)
        }
        Err(InventoryError::MissingPart) => Err(StatusCode::NOT_FOUND),
        Err(InventoryError::GeneralError(_)) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterPartCommand {
    name: String,
}

#[derive(Debug, Serialize)]
struct RegisterPartResponse {
    id: PartId,
}

#[derive(Debug, Serialize)]
struct PartView {
    id: PartId,
    name: String,
}

impl From<Part> for PartView {
    fn from(value: Part) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
        }
    }
}

impl IntoResponse for PartView {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl IntoResponse for RegisterPartResponse {
    fn into_response(self) -> Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/v1/inventory/parts/{}", self.id))],
            Json(self),
        )
            .into_response()
    }
}
