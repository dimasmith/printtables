use crate::projects::domain::name::Name;
use crate::projects::domain::project::{Project, ProjectId};
use crate::projects::domain::repository::ProjectRepository;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::Error;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqlxProjectRepository {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
struct ProjectRecord {
    id: Uuid,
    name: String,
    created_at: NaiveDateTime,
}

impl SqlxProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// TODO: add transactions support
#[async_trait]
impl ProjectRepository for SqlxProjectRepository {
    async fn create(&self, project: Project) -> anyhow::Result<ProjectId> {
        let id = project.id();
        let record = ProjectRecord::from(project);
        sqlx::query!(
            r#"
        insert into project (id, name, created_at)
        values (?, ?, ?)
            "#,
            record.id,
            record.name,
            record.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(id)
    }
    async fn find_by_id(&self, id: ProjectId) -> anyhow::Result<Option<Project>> {
        let result = sqlx::query_as!(
            ProjectRecord,
            r#"
        select id as "id: Uuid", name, created_at 
        from project
        where id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => Ok(Some(Project::from(record))),
            Err(e) => match e {
                Error::RowNotFound => Ok(None),
                _ => Err(anyhow::Error::from(e)),
            },
        }
    }
}

impl From<ProjectRecord> for Project {
    fn from(value: ProjectRecord) -> Self {
        let name = Name::try_from(value.name).unwrap_or_default();
        Project::full(value.id, name, value.created_at)
    }
}

impl From<Project> for ProjectRecord {
    fn from(value: Project) -> Self {
        ProjectRecord {
            id: value.id(),
            name: value.name().to_string(),
            created_at: value.created_at(),
        }
    }
}
