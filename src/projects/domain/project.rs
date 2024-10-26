use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

pub type ProjectId = Uuid;

/// Printable project.
#[derive(Debug, Clone)]
pub struct Project {
    id: ProjectId,
    name: String,
    created_at: NaiveDateTime,
}

impl Project {
    pub fn new(name: String) -> Self {
        let id = Uuid::now_v7();
        let created_at = Local::now().naive_local();
        Self {
            id,
            name,
            created_at,
        }
    }
}

impl Project {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

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
