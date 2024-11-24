use async_trait::async_trait;

use super::project::{Project, ProjectId};

/// Repository for manipulating persistent projects.
/// This trait must be implemented in the infrastructure layer.
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Store a new instance of project in storage.
    async fn create(&self, project: Project) -> anyhow::Result<ProjectId>;

    /// Find project in storage by ID.
    /// The implementation must return Ok(None) if the project is not found.
    /// The Err(_) result is for underlying storage communication errors.
    async fn find_by_id(&self, id: ProjectId) -> anyhow::Result<Option<Project>>;
}
