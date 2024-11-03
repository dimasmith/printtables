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
use crate::server::rest::ErrorResponse;

pub async fn register_part(
    State(inventory): State<Arc<dyn InventoryService>>,
    Json(command): Json<RegisterPartCommand>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let name = parse_register_part_command(command)?;
    let result = inventory.register_part(name).await;
    match result {
        Ok(part_id) => Ok(RegisterPartResponse { id: part_id }),
        Err(_) => Err(ErrorResponse::InternalError),
    }
}

pub async fn view_part(
    State(inventory): State<Arc<dyn InventoryService>>,
    Path(part_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let result = inventory.view_part(part_id).await;
    match result {
        Ok(part) => Ok(PartView::from(part)),
        Err(InventoryError::MissingPart) => Err(ErrorResponse::NotFound),
        Err(InventoryError::GeneralError(_)) => Err(ErrorResponse::InternalError),
    }
}

fn parse_register_part_command(payload: RegisterPartCommand) -> Result<Name, ErrorResponse> {
    match Name::try_from(payload.name) {
        Ok(name) => Ok(name),
        Err(validation) => Err(ErrorResponse::ValidationFailed(vec![validation])),
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
