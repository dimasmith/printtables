use anyhow::anyhow;
use fake::{faker::name::en::Name, Fake};
use reqwest::{Client, Response};
use serde::Serialize;

use crate::server::TestServer;

pub async fn create_project(
    test_server: &TestServer,
    rest_client: &Client,
    payload: &CreateProjectPayload,
) -> anyhow::Result<Response> {
    let url = test_server.uri("/v1/projects");
    rest_client
        .post(url)
        .json(payload)
        .send()
        .await
        .map_err(|e| anyhow!(e))
}

pub async fn view_project_by_id(
    test_server: &TestServer,
    rest_client: &Client,
    id: &str,
) -> anyhow::Result<Response> {
    let uri = &format!("/v1/projects/{}", id);
    view_project_by_uri(test_server, rest_client, uri).await
}

pub async fn view_project_by_uri(
    test_server: &TestServer,
    rest_client: &Client,
    uri: &str,
) -> anyhow::Result<Response> {
    let url = test_server.uri(uri);
    rest_client.get(url).send().await.map_err(|e| anyhow!(e))
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
