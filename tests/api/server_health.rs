use crate::server::start_test_server;
use reqwest::StatusCode;

#[tokio::test]
async fn health_endpoint_responds_with_200() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let response = test_server
        .api_client
        .get(test_server.uri("/health"))
        .send()
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "health endpoint did not respond with 200 status"
    );
    Ok(())
}
