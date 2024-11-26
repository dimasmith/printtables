use crate::server::start_test_server;
use fake::{faker::name::en::Name, Fake};
use printtables::server::rest::ValidationMessage;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, PartialEq)]
struct PartView {
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct RegisterPartRequest {
    name: String,
}

#[tokio::test]
async fn register_and_view_part() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let project_request = RegisterPartRequest::random();
    let create_part_uri = test_server.uri("/v1/inventory/parts");

    let rest_client = reqwest::Client::new();
    let create_proect_response = rest_client
        .post(create_part_uri)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&project_request)
        .send()
        .await?;

    assert_eq!(
        create_proect_response.status(),
        StatusCode::CREATED,
        "the service did not respond with created status"
    );

    let project_location = create_proect_response
        .headers()
        .get("location")
        .ok_or(anyhow::Error::msg("no part ID in location header"))?
        .to_str()?;

    let part_location = test_server.uri(project_location);
    assert!(
        part_location.contains("/v1/inventory/parts"),
        "incorrect part location"
    );

    let view_part_response = rest_client
        .get(part_location)
        .header("Accept", "application/json")
        .send()
        .await?;

    assert_eq!(
        view_part_response.status(),
        StatusCode::OK,
        "the newly created part must be found"
    );

    let project_view = view_part_response.json::<PartView>().await?;
    assert_eq!(
        &project_view.name, &project_request.name,
        "part name is not the same as created part name"
    );

    Ok(())
}

#[tokio::test]
async fn viewing_missing_part_responds_404() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let fake_id = Uuid::now_v7();
    let part_uri = format!("/v1/inventory/parts/{}", fake_id);
    let project_uri = test_server.uri(&part_uri);

    let rest_client = Client::new();
    let response = rest_client
        .get(project_uri)
        .header("Accept", "application/json")
        .send()
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "service must respond with 404 when viewing missing part"
    );

    Ok(())
}

#[tokio::test]
async fn register_invalid_part() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let project_request = RegisterPartRequest::invalid_name();
    let create_project_uri = test_server.uri("/v1/inventory/parts");

    let rest_client = reqwest::Client::new();
    let resp = rest_client
        .post(create_project_uri)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&project_request)
        .send()
        .await?;

    assert_eq!(
        resp.status(),
        StatusCode::BAD_REQUEST,
        "the service did not reject invalid part payload"
    );

    let err_message: ValidationMessage = resp.json().await?;
    let first_error = err_message
        .errors
        .get(0)
        .expect("error must contain at least one entry");

    assert!(
        first_error.code().starts_with("part.name"),
        "incorrect error code"
    );
    assert_eq!(first_error.attribute(), "name");

    Ok(())
}
impl RegisterPartRequest {
    fn random() -> Self {
        let name = Name().fake();
        Self { name }
    }

    fn invalid_name() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}
