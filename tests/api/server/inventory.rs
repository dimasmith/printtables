use core::str;

use anyhow::anyhow;
use fake::{faker::name::en::Name, Fake};
use reqwest::Response;
use serde::{Deserialize, Serialize};

use super::TestServer;

#[derive(Debug, Serialize)]
pub struct RegisterPartPayload {
    name: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct CreatePartResponsePayload {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ViewPartPayload {
    pub id: String,
    pub name: String,
}

impl TestServer {
    pub async fn register_part(&self, payload: &RegisterPartPayload) -> anyhow::Result<Response> {
        let uri = self.uri("/v1/inventory/parts");
        self.api_client
            .post(uri)
            .json(payload)
            .send()
            .await
            .map_err(|e| anyhow!(e))
    }

    pub async fn view_part_by_uri(&self, part_uri: &str) -> anyhow::Result<Response> {
        let uri = self.uri(part_uri);
        self.api_client
            .get(uri)
            .send()
            .await
            .map_err(|e| anyhow!(e))
    }
}

impl RegisterPartPayload {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for RegisterPartPayload {
    fn default() -> Self {
        let name: String = Name().fake();
        Self { name }
    }
}
