use std::sync::Arc;

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use uuid::Uuid;

use crate::projects::app::service::ProjectError;
use crate::projects::app::service::ProjectsService;
use crate::server::rest::ErrorResponse;

pub async fn view_project(
    State(project_service): State<Arc<dyn ProjectsService>>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let project = project_service.view_project(project_id).await;
    match project {
        Ok(p) => Ok(Json(p)),
        Err(ProjectError::MissingProject) => Err(ErrorResponse::NotFound),
        Err(_) => Err(ErrorResponse::InternalError),
    }
}
