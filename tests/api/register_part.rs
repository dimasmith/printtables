use crate::server::{
    inventory::{RegisterPartPayload, ViewPartPayload},
    start_test_server,
};
use printtables::server::rest::ValidationMessage;
use reqwest::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn register_and_view_part() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let create_part_payload = RegisterPartPayload::default();
    let create_part_response = test_server.register_part(&create_part_payload).await?;

    assert_eq!(
        create_part_response.status(),
        StatusCode::CREATED,
        "the service did not respond with created status"
    );

    let part_uri = create_part_response
        .headers()
        .get("location")
        .ok_or(anyhow::Error::msg("no part ID in location header"))?
        .to_str()?;

    let view_part_response = test_server.view_part_by_uri(part_uri).await?;

    assert_eq!(
        view_part_response.status(),
        StatusCode::OK,
        "the newly created part must be found"
    );

    let project_view = view_part_response.json::<ViewPartPayload>().await?;
    assert_eq!(
        &project_view.name,
        &create_part_payload.name(),
        "part name is not the same as created part name"
    );

    Ok(())
}

#[tokio::test]
async fn viewing_missing_part_responds_404() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let fake_id = Uuid::now_v7();
    let part_uri = format!("/v1/inventory/parts/{}", fake_id);

    let response = test_server.view_part_by_uri(&part_uri).await?;

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

    let register_part_payload = RegisterPartPayload::new("");
    let response = test_server.register_part(&register_part_payload).await?;

    assert_eq!(
        response.status(),
        StatusCode::BAD_REQUEST,
        "the service did not reject invalid part payload"
    );

    let err_message: ValidationMessage = response.json().await?;
    let first_error = err_message
        .errors
        .first()
        .expect("error must contain at least one entry");

    assert!(
        first_error.code().starts_with("part.name"),
        "incorrect error code"
    );
    assert_eq!(first_error.attribute(), "name");

    Ok(())
}
