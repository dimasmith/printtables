use fake::{faker::name::en::Name, Fake};
use printtables::server::rest::ValidationMessage;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use server::start_test_server;
use uuid::Uuid;

mod server;

#[derive(Debug, Deserialize, PartialEq)]
struct ProjectView {
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct RegisterProjectRequest {
    name: String,
}

#[tokio::test]
async fn register_and_view_project() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let project_request = RegisterProjectRequest::random();
    let create_project_uri = test_server.uri("/v1/projects");

    let rest_client = reqwest::Client::new();
    let create_proect_response = rest_client
        .post(create_project_uri)
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
        .ok_or(anyhow::Error::msg("no project ID in location header"))?
        .to_str()?;

    let get_project_url = test_server.uri(project_location);

    let view_project_response = rest_client
        .get(get_project_url)
        .header("Accept", "application/json")
        .send()
        .await?;

    assert_eq!(
        view_project_response.status(),
        StatusCode::OK,
        "the newly created project must be found"
    );

    let project_view = view_project_response.json::<ProjectView>().await?;
    assert_eq!(
        &project_view.name, &project_request.name,
        "project name is not the same as created project name"
    );

    Ok(())
}

#[tokio::test]
async fn viewing_missing_project_responds_404() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let fake_id = Uuid::now_v7();
    let project_uri = format!("/v1/projects/{}", fake_id);
    let project_uri = test_server.uri(&project_uri);

    let rest_client = Client::new();
    let response = rest_client
        .get(project_uri)
        .header("Accept", "application/json")
        .send()
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::NOT_FOUND,
        "service must respond with 404 when viewing missing project"
    );

    Ok(())
}

#[tokio::test]
async fn register_invalid_project() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let project_request = RegisterProjectRequest::invalid_name();
    let create_project_uri = test_server.uri("/v1/projects");

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
        "the service did not reject invalid project payload"
    );

    let err_message: ValidationMessage = resp.json().await?;
    let first_err = err_message
        .errors
        .get(0)
        .expect("validation message must contain entries");

    assert!(
        first_err.code().starts_with("project.name"),
        "incorrect error code"
    );
    assert_eq!(first_err.attribute(), "name");

    Ok(())
}

impl RegisterProjectRequest {
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
