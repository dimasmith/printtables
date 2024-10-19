//! Application router configuration.

use crate::server::routes::health::health;
use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Provide a default router for HTTP requests.
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
