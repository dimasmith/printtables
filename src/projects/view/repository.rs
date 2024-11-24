use async_trait::async_trait;

use crate::projects::domain::project::ProjectId;

use super::project::ProjectView;

#[async_trait]
pub trait ProjectViewRepository: Send + Sync {
    /// Retrieves project and populates the project view.
    ///
    /// Returns Ok(None) when project with the given ID does not exist.
    async fn get_view_by_id(&self, id: ProjectId) -> anyhow::Result<Option<ProjectView>>;
}
