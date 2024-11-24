//! Defines main application service for Projects.

use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, info};

use crate::projects::domain::name::Name;
use crate::projects::domain::project::{Project, ProjectId};
use crate::projects::domain::repository::ProjectRepository;
use crate::projects::view::project::ProjectView;
use crate::projects::view::repository::ProjectViewRepository;

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
    async fn register_project(&self, name: Name) -> Result<ProjectId, ProjectError>;

    /// View the project with identifier id.
    async fn view_project(&self, id: ProjectId) -> Result<ProjectView, ProjectError>;
}

pub struct DefaultProjectService<R: ProjectRepository, V: ProjectViewRepository> {
    projects_repo: Arc<R>,
    view_repo: Arc<V>,
}

impl<R: ProjectRepository, V: ProjectViewRepository> DefaultProjectService<R, V> {
    pub fn new(repo: Arc<R>, view_repo: Arc<V>) -> Self {
        Self {
            projects_repo: repo,
            view_repo,
        }
    }
}

#[async_trait]
impl<R: ProjectRepository, V: ProjectViewRepository> ProjectsService
    for DefaultProjectService<R, V>
{
    async fn register_project(&self, name: Name) -> Result<ProjectId, ProjectError> {
        // TODO: check if it's possible to avoid cloning here.
        let new_project = Project::new(name.clone());
        let repo = &self.projects_repo;
        let project_id = repo.create(new_project).await?;
        info!("created project {} with ID {}", name, &project_id);
        Ok(project_id)
    }

    async fn view_project(&self, id: ProjectId) -> Result<ProjectView, ProjectError> {
        let repo = &self.view_repo;
        let project = repo
            .get_view_by_id(id)
            .await
            .map_err(ProjectError::GeneralError);
        if project.is_err() {
            error!("project retrieval failed: {:?}", project);
        }
        match project? {
            Some(p) => Ok(p),
            None => Err(ProjectError::MissingProject),
        }
    }
}
