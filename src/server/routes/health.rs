//! Status checks of running server.

/// Sends OK response if the service is up and running and ready to receive requests.
pub async fn health() -> String {
    "OK".to_string()
}