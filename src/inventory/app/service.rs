use std::sync::Arc;

use axum::async_trait;
use thiserror::Error;
use tracing::info;

use crate::inventory::domain::{
    name::Name,
    part::{Part, PartId},
    part_repository::PartRepository,
};

#[derive(Debug, Error)]
pub enum InventoryError {
    #[error("requested part not found")]
    MissingPart,
    #[error("general error")]
    GeneralError(anyhow::Error),
}

#[async_trait]
pub trait InventoryService: Sync + Send {
    /// Registers a new part in the inventory.
    async fn register_part(&self, name: Name) -> Result<PartId, anyhow::Error>;

    async fn view_part(&self, id: PartId) -> Result<Part, InventoryError>;
}

#[derive(Debug)]
pub struct DefaultInventoryService {
    parts_repo: Arc<dyn PartRepository>,
}

impl DefaultInventoryService {
    pub fn new(parts_repo: Arc<dyn PartRepository>) -> Self {
        Self { parts_repo }
    }
}

#[async_trait]
impl InventoryService for DefaultInventoryService {
    async fn register_part(&self, name: Name) -> Result<PartId, anyhow::Error> {
        let new_part = Part::new(name.clone());
        let id = new_part.id();
        self.parts_repo.insert(new_part).await?;
        info!("part {} registered with id {}", &name, id);
        Ok(id)
    }

    async fn view_part(&self, id: PartId) -> Result<Part, InventoryError> {
        let result = self.parts_repo.find_by_id(id).await;
        match result {
            Ok(Some(part)) => Ok(part),
            Ok(None) => Err(InventoryError::MissingPart),
            Err(e) => Err(InventoryError::GeneralError(e)),
        }
    }
}
