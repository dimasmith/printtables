//! Provides implementation of project view repository.

use anyhow::anyhow;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::projects::{
    domain::project::ProjectId,
    view::{project::ProjectView, repository::ProjectViewRepository},
};

pub struct SqlxProjectViewRepository {
    pool: SqlitePool,
}

impl SqlxProjectViewRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectViewRepository for SqlxProjectViewRepository {
    async fn get_view_by_id(&self, id: ProjectId) -> anyhow::Result<Option<ProjectView>> {
        let result = sqlx::query!(
            r#"
        select id as "id: Uuid", name
        from project
        where id = ?
        "#,
            id
        )
        .fetch_one(&self.pool)
        .await;
        match result {
            Ok(record) => Ok(Some(ProjectView::new(record.id, record.name))),
            Err(sqlx::Error::RowNotFound) => Ok(None),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
