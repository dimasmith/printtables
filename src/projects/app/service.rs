//! Defines main application service for Projects.

use async_trait::async_trait;
use core::str;
use std::sync::Arc;
use tracing::info;

use crate::projects::domain::project::{Project, ProjectId, ProjectRepository};

#[async_trait]
pub trait ProjectsService: Send + Sync {
    /// Register a new project in the system.
    async fn register_project(&self, name: &str) -> anyhow::Result<ProjectId>;

    /// View the project with identifier id.
    async fn view_project(&self, id: ProjectId) -> anyhow::Result<Project>;
}

pub struct DefaultProjectService<R: ProjectRepository> {
    projects_repo: Arc<R>,
}

impl<R: ProjectRepository> DefaultProjectService<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self {
            projects_repo: repo,
        }
    }
}

#[async_trait]
impl<R: ProjectRepository> ProjectsService for DefaultProjectService<R> {
    async fn register_project(&self, name: &str) -> anyhow::Result<ProjectId> {
        let new_project = Project::new(name.to_string());
        let repo = &self.projects_repo;
        let project_id = repo.create(new_project).await?;
        info!("created project {} with ID {}", name, &project_id);
        Ok(project_id)
    }

    async fn view_project(&self, id: ProjectId) -> anyhow::Result<Project> {
        let repo = &self.projects_repo;
        let project = repo.find_by_id(id).await?;
        project.ok_or_else(|| anyhow::Error::msg("project not found"))
    }
}
