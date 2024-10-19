//! Starts the server.

use crate::server::routes::router::router;
use tokio::net::TcpListener;

pub async fn start_server(listener: TcpListener) -> anyhow::Result<()> {
    let app = router();
    axum::serve(listener, app).await?;
    Ok(())

}