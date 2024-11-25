//! Manipulates project parts (BOM).

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

use crate::inventory::domain::part::PartId;
use crate::projects::app::service::ProjectsService;
use crate::projects::domain::project::{ProjectId, ProjectPart};
use crate::server::rest::ErrorResponse;

pub async fn define_project_bom(
    State(project_service): State<Arc<dyn ProjectsService>>,
    Path(project_id): Path<ProjectId>,
    Json(parts): Json<PartsPayload>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let bom = parts
        .parts
        .into_iter()
        .map(|p| ProjectPart::new(p.part, p.quantity))
        .collect();
    let result = project_service.set_project_bom(project_id, bom).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(ErrorResponse::InternalError),
    }
}

#[derive(Debug, Deserialize)]
pub struct PartsPayload {
    parts: Vec<PartPayload>,
}

#[derive(Debug, Deserialize)]
pub struct PartPayload {
    part: PartId,
    quantity: u32,
}
