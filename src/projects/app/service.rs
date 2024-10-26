//! Defines main application service for Projects.

use async_trait::async_trait;
use core::str;
use std::sync::Arc;
use thiserror::Error;
use tracing::info;

use crate::projects::domain::project::{Project, ProjectId, ProjectRepository};

/// Typical errors happening during project processing.
#[derive(Debug, Error)]
pub enum ProjectError {
    /// Requested project cannot be found or it was deleted.
    #[error("requested project does not exist")]
    MissingProject,
    /// Wrapper error for errors reported by downstream components.
    #[error("error procesing project")]
    GeneralError(#[from] anyhow::Error),
}

#[async_trait]
pub trait ProjectsService: Send + Sync {
    /// Register a new project in the system.
    async fn register_project(&self, name: &str) -> Result<ProjectId, ProjectError>;

    /// View the project with identifier id.
    async fn view_project(&self, id: ProjectId) -> Result<Project, ProjectError>;
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
    async fn register_project(&self, name: &str) -> Result<ProjectId, ProjectError> {
        let new_project = Project::new(name.to_string());
        let repo = &self.projects_repo;
        let project_id = repo.create(new_project).await?;
        info!("created project {} with ID {}", name, &project_id);
        Ok(project_id)
    }

    async fn view_project(&self, id: ProjectId) -> Result<Project, ProjectError> {
        let repo = &self.projects_repo;
        let project = repo
            .find_by_id(id)
            .await
            .map_err(|e| ProjectError::GeneralError(e))?;
        match project {
            Some(p) => Ok(p),
            None => Err(ProjectError::MissingProject),
        }
    }
}
