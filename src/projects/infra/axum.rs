use std::sync::Arc;

use axum::extract::State;

use crate::projects::app::service::ProjectsService;

pub async fn register_project(State(project_service): State<Arc<dyn ProjectsService>>) {
    project_service.register_project("Test").await;
}
