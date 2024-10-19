//! Defines main application service for Projects.

use core::str;

use crate::projects::domain::project::ProjectId;

#[allow(async_fn_in_trait)]
pub trait ProjectsService {
    /// Register a new project in the system.
    async fn register_project(name: &str) -> anyhow::Result<ProjectId>;

    /// View the project with identifier id.
    async fn view_project(id: ProjectId) -> anyhow::Result<()>;
}
