//! View model for the project. Useful for displaying project details to users.

use serde::Serialize;

use crate::{inventory::domain::part::PartId, projects::domain::project::ProjectId};

#[derive(Debug, Clone, Serialize)]
pub struct ProjectView {
    id: ProjectId,
    name: String,
    bom: Vec<ProjectPart>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProjectPart {
    part_id: PartId,
    name: String,
    quantity: u32,
}

impl ProjectView {
    pub fn new(id: ProjectId, name: String, parts: Vec<ProjectPart>) -> Self {
        Self {
            id,
            name,
            bom: parts,
        }
    }
}

impl ProjectPart {
    pub fn new(part_id: PartId, name: String, quantity: u32) -> Self {
        Self {
            part_id,
            name,
            quantity,
        }
    }
}

impl ProjectView {
    pub fn id(&self) -> ProjectId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bom_size(&self) -> usize {
        self.bom.len()
    }

    pub fn parts(&self) -> &[ProjectPart] {
        self.bom.as_slice()
    }
}
