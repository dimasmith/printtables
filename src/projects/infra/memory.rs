//! Temporary module containing in-memory implementation
//! of projects repository.
//! It will be replaced with sqlx-based implementation later.

use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::projects::domain::project::{Project, ProjectId, ProjectRepository};

#[derive(Debug, Default)]
pub struct InMemoryProjectsRepository {
    storage: Mutex<HashMap<ProjectId, Project>>,
}

#[async_trait]
impl ProjectRepository for InMemoryProjectsRepository {
    async fn create(&self, project: Project) -> anyhow::Result<ProjectId> {
        let mut storage = self.storage.lock().await;
        let id = project.id();
        storage.insert(id, project);
        Ok(id)
    }

    async fn find_by_id(&self, id: ProjectId) -> anyhow::Result<Option<Project>> {
        let storage = self.storage.lock().await;
        let project = storage.get(&id);
        Ok(project.cloned())
    }
}
