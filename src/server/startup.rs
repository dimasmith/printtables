//! Starts the server.

use std::sync::{Arc, LazyLock};

use crate::projects::app::service::{DefaultProjectService, ProjectsService};
use crate::projects::infra::memory::InMemoryProjectsRepository;
use crate::server::routes::router::router;
use tokio::net::TcpListener;

use super::tracing::initialize_tracing;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    initialize_tracing();
});

pub async fn start_server(listener: TcpListener) -> anyhow::Result<()> {
    LazyLock::force(&TRACING);

    let project_repo = InMemoryProjectsRepository::default();
    let project_service = DefaultProjectService::new(Arc::new(project_repo));
    let shared_project_service: Arc<dyn ProjectsService> = Arc::new(project_service);

    let app = router(shared_project_service);
    axum::serve(listener, app).await?;
    Ok(())
}
