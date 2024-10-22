//! Application router configuration.

use std::sync::Arc;

use crate::projects::app::service::DefaultProjectService;
use crate::projects::infra::axum::{register_project, view_project};
use crate::projects::infra::memory::InMemoryProjectsRepository;
use crate::server::routes::health::health;
use axum::routing::{get, post};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Provide a default router for HTTP requests.
pub fn router() -> Router {
    let project_repo = InMemoryProjectsRepository::default();
    let project_service = DefaultProjectService::new(Arc::new(project_repo));

    Router::new()
        .route("/health", get(health))
        .route("/v1/projects", post(register_project))
        .route("/v1/projects/:project_id", get(view_project))
        .with_state(Arc::new(project_service))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
