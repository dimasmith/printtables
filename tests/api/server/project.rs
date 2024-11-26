use anyhow::anyhow;
use fake::{faker::name::en::Name, Fake};
use reqwest::Response;
use serde::Serialize;

use crate::server::TestServer;

impl TestServer {
    pub async fn create_project(&self, payload: &CreateProjectPayload) -> anyhow::Result<Response> {
        let url = self.uri("/v1/projects");
        self.api_client
            .post(url)
            .json(payload)
            .send()
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn view_project_by_uri(&self, uri: &str) -> anyhow::Result<Response> {
        let url = self.uri(uri);
        self.api_client
            .get(url)
            .send()
            .await
            .map_err(|e| anyhow!(e))
    }
}

#[derive(Debug, Serialize)]
pub struct CreateProjectPayload {
    name: String,
}

impl CreateProjectPayload {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for CreateProjectPayload {
    fn default() -> Self {
        let name: String = Name().fake();
        Self::new(name)
    }
}
