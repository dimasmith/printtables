//! Application router configuration.

use std::sync::Arc;

use crate::inventory::app::service::InventoryService;
use crate::projects::app::service::ProjectsService;
use crate::server::routes::health::health;
use crate::server::routes::project::parts::define_project_bom;
use crate::server::routes::project::register::register_project;
use crate::server::routes::project::view::view_project;
use axum::routing::{get, post, put};
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use super::inventory::parts::{register_part, view_part};

/// Provide a default router for HTTP requests.
pub fn router(
    project_service: Arc<dyn ProjectsService>,
    inventory: Arc<dyn InventoryService>,
) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/v1/projects", post(register_project))
        .route("/v1/projects/:project_id", get(view_project))
        .route("/v1/projects/:project_id/parts", put(define_project_bom))
        .with_state(Arc::clone(&project_service))
        .route("/v1/inventory/parts", post(register_part))
        .route("/v1/inventory/parts/:part_id", get(view_part))
        .with_state(Arc::clone(&inventory))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
