//! The repository trait to interact with inventory.
use std::fmt::Debug;

use async_trait::async_trait;

use super::part::{Part, PartId};

#[async_trait]
pub trait PartRepository: Debug + Send + Sync {
    /// Inserts a new part into the storage.
    async fn insert(&self, part: Part) -> anyhow::Result<()>;

    async fn find_by_id(&self, id: PartId) -> anyhow::Result<Option<Part>>;
}
