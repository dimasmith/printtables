use std::sync::Arc;

use axum::extract::{self, Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::projects::app::service::ProjectError;
use crate::projects::domain::name::Name;
use crate::projects::view::project::ProjectView;
use crate::projects::{app::service::ProjectsService, domain::project::ProjectId};
use crate::server::rest::ErrorResponse;
use crate::shared::validation::validator::CollectingValidator;

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterProjectCommand {
    name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectCreatedResponse {
    id: ProjectId,
}

pub async fn register_project(
    State(project_service): State<Arc<dyn ProjectsService>>,
    extract::Json(command): extract::Json<RegisterProjectCommand>,
) -> Result<ProjectCreatedResponse, ErrorResponse> {
    let name = parse_create_request(command)?;
    let result = project_service.register_project(name).await;
    match result {
        Ok(id) => Ok(ProjectCreatedResponse { id }),
        Err(_) => Err(ErrorResponse::InternalError),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectViewPayload {
    id: ProjectId,
    name: String,
}

pub async fn view_project(
    State(project_service): State<Arc<dyn ProjectsService>>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ErrorResponse> {
    let project = project_service.view_project(project_id).await;
    match project {
        Ok(p) => Ok(Json(ProjectViewPayload::from(p))),
        Err(ProjectError::MissingProject) => Err(ErrorResponse::NotFound),
        Err(_) => Err(ErrorResponse::InternalError),
    }
}

impl IntoResponse for ProjectCreatedResponse {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::CREATED,
            [(header::LOCATION, format!("/v1/projects/{}", self.id))],
            Json(self),
        )
            .into_response()
    }
}

impl From<ProjectView> for ProjectViewPayload {
    fn from(value: ProjectView) -> Self {
        let id = value.id();
        let name = value.name().to_string();
        ProjectViewPayload { id, name }
    }
}

fn parse_create_request(payload: RegisterProjectCommand) -> Result<Name, ErrorResponse> {
    let mut validator = CollectingValidator::default();
    let name = validator.parse_string::<Name>(payload.name);

    if validator.has_errors() {
        Err(ErrorResponse::ValidationFailed(validator.into_errors()))
    } else {
        Ok(name.unwrap())
    }
}
