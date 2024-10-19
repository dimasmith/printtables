use reqwest::StatusCode;
use crate::server::start_test_server;

mod server;

#[tokio::test]
async fn health_endpoint_responds_with_200() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let client = reqwest::Client::new();
    let response = client.get(&test_server.uri("/health")).send().await?;

    assert_eq!(response.status(), StatusCode::OK, "health endpoint did not respond with 200 status");
    Ok(())
}