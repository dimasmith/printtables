//! Starts the server.

use crate::server::routes::router::router;
use tokio::net::TcpListener;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub async fn start_server(listener: TcpListener) -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::registry()
        .with(LevelFilter::from_level(Level::TRACE))
        .with(fmt::Layer::default());

    subscriber.init();

    let app = router();
    axum::serve(listener, app).await?;
    Ok(())
}
