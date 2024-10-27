use std::sync::Arc;

use axum::extract::{self, Path, State};
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::projects::app::service::ProjectError;
use crate::projects::domain::name::Name;
use crate::projects::domain::project::Project;
use crate::projects::domain::validation::ValidationError;
use crate::projects::{app::service::ProjectsService, domain::project::ProjectId};

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
) -> Result<ProjectCreatedResponse, impl IntoResponse> {
    let name = match Name::try_from(command.name) {
        Ok(name) => name,
        Err(err) => {
            return Err(err.into_response());
        }
    };
    let id = project_service.register_project(name).await;
    if let Err(e) = id {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)).into_response());
    }

    let id = id.unwrap();
    Ok(ProjectCreatedResponse { id })
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectResponse {
    id: ProjectId,
    name: String,
}

pub async fn view_project(
    State(project_service): State<Arc<dyn ProjectsService>>,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, StatusCode> {
    let project = project_service.view_project(project_id).await;
    match project {
        Ok(p) => Ok(Json(ProjectResponse::from(p))),
        Err(e) => match e {
            ProjectError::MissingProject => Err(StatusCode::NOT_FOUND),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
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

impl IntoResponse for ValidationError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(self)).into_response()
    }
}

impl From<Project> for ProjectResponse {
    fn from(value: Project) -> Self {
        let id = value.id();
        let name = value.name().to_string();
        ProjectResponse { id, name }
    }
}
