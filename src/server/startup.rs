//! Starts the server.

use std::sync::{Arc, LazyLock};

use crate::infra::sqlite::part::SqlitePartRepository;
use crate::inventory::app::service::DefaultInventoryService;
use crate::projects::app::service::{DefaultProjectService, ProjectsService};
use crate::projects::infra::sqlx::project_repository::SqlxProjectRepository;
use crate::server::routes::router::router;
use sqlx::{migrate, SqlitePool};
use tokio::net::TcpListener;

use super::tracing::initialize_tracing;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    initialize_tracing();
});

pub async fn start_server(listener: TcpListener, db_pool: SqlitePool) -> anyhow::Result<()> {
    LazyLock::force(&TRACING);

    // run database migrations
    migrate!("./migrations").run(&db_pool).await?;

    let project_repo = SqlxProjectRepository::new(db_pool.clone());
    let project_service = DefaultProjectService::new(Arc::new(project_repo));
    let shared_project_service: Arc<dyn ProjectsService> = Arc::new(project_service);

    let parts_repo = SqlitePartRepository::new(db_pool.clone());
    let inventory = DefaultInventoryService::new(Arc::new(parts_repo));

    let app = router(Arc::clone(&shared_project_service), Arc::new(inventory));
    axum::serve(listener, app).await?;
    Ok(())
}
