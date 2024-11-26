use crate::server::{project::CreateProjectPayload, start_test_server};
use printtables::{projects::view::project::ProjectView, server::rest::ValidationMessage};
use reqwest::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn register_and_view_project() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let project_request = CreateProjectPayload::default();

    let create_proect_response = test_server.create_project(&project_request).await?;

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

    let view_project_response = test_server.view_project_by_uri(project_location).await?;

    assert_eq!(
        view_project_response.status(),
        StatusCode::OK,
        "the newly created project must be found"
    );

    let project_view = view_project_response.json::<ProjectView>().await?;
    assert_eq!(
        &project_view.name(),
        &project_request.name(),
        "project name is not the same as created project name"
    );

    Ok(())
}

#[tokio::test]
async fn viewing_missing_project_responds_404() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let fake_id = Uuid::now_v7();
    let project_uri = format!("/v1/projects/{}", fake_id);

    let response = test_server.view_project_by_uri(&project_uri).await?;

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

    let project_with_empty_name = CreateProjectPayload::new("");
    let response = test_server.create_project(&project_with_empty_name).await?;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "the service did not reject invalid project payload"
    );

    let err_message: ValidationMessage = response.json().await?;
    let first_err = err_message
        .errors
        .first()
        .expect("validation message must contain entries");

    assert!(
        first_err.code().starts_with("project.name"),
        "incorrect error code"
    );
    assert_eq!(first_err.attribute(), "name");

    Ok(())
}
