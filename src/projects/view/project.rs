//! View model for the project. Useful for displaying project details to users.

use crate::projects::domain::project::ProjectId;

#[derive(Debug, Clone)]
pub struct ProjectView {
    id: ProjectId,
    name: String,
    bom_size: usize,
}

impl ProjectView {
    pub fn new(id: ProjectId, name: String) -> Self {
        Self {
            id,
            name,
            bom_size: 0,
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
        self.bom_size
    }
}
