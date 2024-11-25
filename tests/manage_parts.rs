use fake::{faker::name::en::Name, Fake};
use printtables::projects::view::project::ProjectView;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use server::{start_test_server, TestServer};

mod server;

#[derive(Debug, Serialize)]
struct RegisterProjectRequest {
    name: String,
}

#[derive(Debug, Serialize)]
struct RegisterPartRequest {
    name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PartView {
    id: String,
    name: String,
}

#[tokio::test]
async fn register_project_and_set_parts() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;
    let rest_client = reqwest::Client::new();
    let project_resource = create_project(&test_server).await?;
    let test_part = RegisterPartRequest::random();
    let part_id = create_part(&test_server, &test_part).await?;

    let project_view = view_project(&test_server, &rest_client, &project_resource).await?;
    assert_eq!(
        project_view.parts().len(),
        0,
        "the project must have no parts after creation, but it wasn't empty"
    );

    let quantity: u32 = (1..100).fake();
    let update_parts_url = test_server.uri(&format!("{}/parts", project_resource));
    let update_parts_response = rest_client
        .put(update_parts_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(format!(
            r#"
        {{ 
            "parts": [
                {{ "part": "{}", "quantity": {} }}
            ]
        }}
        "#,
            part_id, quantity
        ))
        .send()
        .await?;

    assert_eq!(update_parts_response.status(), reqwest::StatusCode::OK);

    let project_view = view_project(&test_server, &rest_client, &project_resource).await?;
    assert_eq!(
        project_view.parts().len(),
        1,
        "the project must contain parts that were added"
    );
    let part = &project_view.parts()[0];
    assert_eq!(part.id().to_string(), part_id);
    assert_eq!(part.quantity(), quantity);
    assert_eq!(part.name(), &test_part.name);

    Ok(())
}

async fn view_project(
    test_server: &TestServer,
    rest_client: &reqwest::Client,
    project_location: &str,
) -> anyhow::Result<ProjectView> {
    let get_project_url = test_server.uri(project_location);
    let view_project_response = rest_client.get(get_project_url).send().await?;
    let project_view: ProjectView = view_project_response.json().await?;
    Ok(project_view)
}

async fn create_project(test_server: &TestServer) -> anyhow::Result<String> {
    let project_request = RegisterProjectRequest::random();
    let create_project_uri = test_server.uri("/v1/projects");

    let rest_client = reqwest::Client::new();
    let create_project_response = rest_client
        .post(create_project_uri)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&project_request)
        .send()
        .await?;

    assert_eq!(
        create_project_response.status(),
        StatusCode::CREATED,
        "the service did not respond with created status"
    );

    Ok(create_project_response
        .headers()
        .get("location")
        .ok_or(anyhow::Error::msg("no project ID in location header"))?
        .to_str()
        .map(|s| s.to_string())?)
}

async fn create_part(
    test_server: &TestServer,
    part_request: &RegisterPartRequest,
) -> anyhow::Result<String> {
    let create_part_uri = test_server.uri("/v1/inventory/parts");

    let rest_client = reqwest::Client::new();
    let create_proect_response = rest_client
        .post(create_part_uri)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&part_request)
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

    let part_view = view_part_response.json::<PartView>().await?;
    assert_eq!(
        &part_view.name, &part_request.name,
        "part name is not the same as created part name"
    );

    Ok(part_view.id)
}

impl RegisterProjectRequest {
    fn random() -> Self {
        let name = Name().fake();
        Self { name }
    }
}

impl RegisterPartRequest {
    fn random() -> Self {
        let name = Name().fake();
        Self { name }
    }
}
