//! Part domain entity.
//! Represents a single printable part that can be added to a project and placed on a table.

use uuid::Uuid;

use super::name::Name;

#[derive(Debug)]
pub struct Part {
    id: PartId,
    name: Name,
}

pub type PartId = Uuid;

impl Part {
    /// To use only with the database deserializers.
    pub fn hydrate(id: PartId, name: Name) -> Self {
        Self { id, name }
    }

    /// Create a new named part.
    pub fn new(name: Name) -> Self {
        let id = Uuid::now_v7();
        Part::hydrate(id, name)
    }

    pub fn id(&self) -> PartId {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }
}
