//! Provides implementation of project view repository.

use anyhow::bail;
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::projects::domain::project::ProjectId;
use crate::projects::view::project::ProjectPart;
use crate::projects::view::{project::ProjectView, repository::ProjectViewRepository};

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
        let project_result = sqlx::query!(
            r#"
            select id as "id: Uuid", name 
            from project
            where id = ?
        "#,
            id
        )
        .fetch_optional(&self.pool)
        .await;

        let project_record = match project_result {
            Ok(Some(record)) => record,
            Ok(None) => return Ok(None),
            Err(e) => bail!(e),
        };

        let bom_result = sqlx::query!(
            r#"
            select bom.part_id as "part_id: Uuid", bom.quantity as "quantity: u32", part.name as part_name 
            from bom 
            join part on part.id = bom.part_id
            where bom.project_id = ?
        "#,
            id
        )
        .fetch_all(&self.pool)
        .await;

        let bom_records = match bom_result {
            Ok(parts) => parts,
            Err(e) => bail!(e),
        };

        let parts = bom_records
            .into_iter()
            .map(|record| ProjectPart::new(record.part_id, record.part_name, record.quantity))
            .collect();
        let project_view = ProjectView::new(project_record.id, project_record.name, parts);
        Ok(Some(project_view))
    }
}
