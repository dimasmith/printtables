use async_trait::async_trait;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

use crate::inventory::domain::name::Name;
use crate::inventory::domain::part::{Part, PartId};
use crate::inventory::domain::part_repository::PartRepository;

#[derive(Debug)]
pub struct SqlxPartRepository {
    pool: SqlitePool,
}

impl SqlxPartRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PartRepository for SqlxPartRepository {
    async fn insert(&self, part: Part) -> anyhow::Result<()> {
        let record = PartRecord::from(part);
        sqlx::query!(
            r#"
        insert into part (id, name)
        values (?, ?)
            "#,
            record.id,
            record.name
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: PartId) -> anyhow::Result<Option<Part>> {
        let result = sqlx::query_as!(
            PartRecord,
            r#"
            select id as "id: Uuid", name 
            from part 
            where id = ?
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await;

        // TODO: extract helper function to hanle missing items.
        match result {
            Ok(record) => {
                let part = Part::try_from(record).map_err(anyhow::Error::from)?;
                Ok(Some(part))
            }
            Err(e) => match e {
                Error::RowNotFound => Ok(None),
                _ => Err(anyhow::Error::from(e)),
            },
        }
    }
}

#[derive(Debug)]
struct PartRecord {
    id: Uuid,
    name: String,
}

impl TryFrom<PartRecord> for Part {
    type Error = anyhow::Error;

    fn try_from(value: PartRecord) -> Result<Self, Self::Error> {
        let name = Name::try_from(value.name).map_err(anyhow::Error::from)?;
        Ok(Part::hydrate(value.id, name))
    }
}

impl From<Part> for PartRecord {
    fn from(value: Part) -> Self {
        Self {
            id: value.id(),
            name: value.name().to_string(),
        }
    }
}
