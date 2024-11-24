use super::name::Name;
use async_trait::async_trait;
use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

pub type ProjectId = Uuid;

/// Printable project.
#[derive(Debug, Clone)]
pub struct Project {
    id: ProjectId,
    name: Name,
    created_at: NaiveDateTime,
}

impl Project {
    pub fn new(name: Name) -> Self {
        let id = Uuid::now_v7();
        let created_at = Local::now().naive_local();
        Project::full(id, name, created_at)
    }

    pub fn full(id: ProjectId, name: Name, created_at: NaiveDateTime) -> Self {
        Self {
            id,
            name,
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

    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
}
