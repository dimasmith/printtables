use std::vec;

use crate::inventory::domain::part::PartId;

use super::name::Name;
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

pub type ProjectId = Uuid;

/// Printable project.
#[derive(Debug, Clone)]
pub struct Project {
    id: ProjectId,
    name: Name,
    parts: Vec<ProjectPart>,
    created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ProjectPart {
    part: PartId,
    quantity: u32,
}

impl Project {
    pub fn new(name: Name) -> Self {
        let id = Uuid::now_v7();
        let created_at = Local::now().naive_local();
        Project::full(id, name, vec![], created_at)
    }

    pub fn full(
        id: ProjectId,
        name: Name,
        parts: Vec<ProjectPart>,
        created_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            name,
            parts,
            created_at,
        }
    }
}

impl Project {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn parts(&self) -> &[ProjectPart] {
        self.parts.as_slice()
    }

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}

impl Project {
    /// Specify BOM for the project.
    pub fn define_parts(&mut self, parts: Vec<ProjectPart>) {
        self.parts = parts;
    }
}

impl ProjectPart {
    pub fn new(part: PartId, quantity: u32) -> Self {
        Self { part, quantity }
    }
}

impl ProjectPart {
    pub fn part(&self) -> PartId {
        self.part
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }
}
