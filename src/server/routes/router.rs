use axum::Router;
use axum::routing::get;
use crate::server::routes::health::health;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
}