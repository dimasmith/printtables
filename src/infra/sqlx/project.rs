use crate::projects::domain::name::Name;
use crate::projects::domain::project::{Project, ProjectId, ProjectPart};
use crate::projects::domain::repository::ProjectRepository;
use anyhow::bail;
use async_trait::async_trait;
use chrono::NaiveDateTime;
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
        .fetch_optional(&self.pool)
        .await;

        let project_record = match result {
            Ok(Some(record)) => record,
            Ok(None) => return Ok(None),
            Err(e) => bail!(e),
        };

        let bom_result = sqlx::query!(
            r#"
        select bom.part_id as "part_id: Uuid", bom.quantity as "quantity: u32"
        from bom
        where bom.project_id = ?
        "#,
            id
        )
        .fetch_all(&self.pool)
        .await;

        let parts_records = match bom_result {
            Ok(records) => records,
            Err(e) => bail!(e),
        };

        let parts = parts_records
            .into_iter()
            .map(|p| ProjectPart::new(p.part_id, p.quantity))
            .collect();
        let project = Project::full(
            project_record.id,
            Name::try_from(project_record.name).unwrap_or_default(),
            parts,
            project_record.created_at,
        );
        Ok(Some(project))
    }

    async fn update(&self, project: Project) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;
        let id = project.id();
        let name = project.name().to_string();
        let _ = sqlx::query!(
            r#"
        update project set name = ? where id = ?
        "#,
            id,
            name,
        )
        .execute(&mut *tx)
        .await;

        let _ = sqlx::query!(r#"delete from bom where project_id = ?"#, id)
            .execute(&mut *tx)
            .await;

        for part in project.parts() {
            let part_id = part.part();
            let quantity = part.quantity();
            let _ = sqlx::query!(
                r#"
            insert into bom (project_id, part_id, quantity) values (?, ?, ?)
            "#,
                id,
                part_id,
                quantity
            )
            .execute(&mut *tx)
            .await;
        }

        tx.commit().await?;
        Ok(())
    }
}

impl From<ProjectRecord> for Project {
    fn from(value: ProjectRecord) -> Self {
        let name = Name::try_from(value.name).unwrap_or_default();
        Project::full(value.id, name, vec![], value.created_at)
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
