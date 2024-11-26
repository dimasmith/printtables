use crate::server::project::CreateProjectPayload;
use crate::server::{start_test_server, TestServer};
use fake::{faker::name::en::Name, Fake};
use printtables::projects::view::project::ProjectView;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

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

    let project_uri = given_new_project(&test_server).await?;
    ensure_project_have_no_parts(&test_server, &project_uri).await?;
    let test_part = RegisterPartRequest::random();
    let part_id = create_part(&test_server, &test_part).await?;

    let quantity: u32 = (1..100).fake();
    let update_parts_url = test_server.uri(&format!("{}/parts", project_uri));
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

    let project_view = view_project(&test_server, &project_uri).await?;
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

async fn ensure_project_have_no_parts(
    test_server: &TestServer,
    project_uri: &str,
) -> anyhow::Result<()> {
    let response = test_server.view_project_by_uri(project_uri).await?;
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "failed to retrieve a project {}",
        project_uri
    );
    let view: ProjectView = response.json().await?;
    assert!(
        view.parts().is_empty(),
        "the base projects must not have any parts"
    );
    Ok(())
}

async fn view_project(test_server: &TestServer, project_uri: &str) -> anyhow::Result<ProjectView> {
    let view_project_response = test_server.view_project_by_uri(project_uri).await?;
    let project_view: ProjectView = view_project_response.json().await?;
    Ok(project_view)
}

async fn given_new_project(test_server: &TestServer) -> anyhow::Result<String> {
    let create_project_payload = CreateProjectPayload::default();
    let response = test_server.create_project(&create_project_payload).await?;
    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "failed to create a project"
    );

    Ok(response
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

impl RegisterPartRequest {
    fn random() -> Self {
        let name = Name().fake();
        Self { name }
    }
}
