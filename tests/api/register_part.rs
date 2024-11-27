use crate::server::{
    inventory::{CreatePartResponsePayload, RegisterPartPayload, ViewPartPayload},
    rest::{CreatedResponse, OkResponse},
    start_test_server,
};
use printtables::server::rest::ValidationMessage;
use reqwest::StatusCode;
use uuid::Uuid;

type CreateResponse = CreatedResponse<CreatePartResponsePayload>;
type ViewPartResponse = OkResponse<ViewPartPayload>;

#[tokio::test]
async fn register_and_view_part() -> anyhow::Result<()> {
    let test_server = start_test_server().await?;

    let create_part_payload = RegisterPartPayload::default();
    let response = test_server.register_part(&create_part_payload).await?;
    let create_response = CreateResponse::from(response).await;

    let payload = create_response.payload();
    assert!(
        !payload.id.is_empty(),
        "missing part identifier on creation"
    );

    let part_uri = &create_response.location;

    let response = test_server.view_part_by_uri(part_uri).await?;
    let view_part_response = ViewPartResponse::from(response).await;

    let view_part_payload = view_part_response.payload();
    assert_eq!(
        view_part_payload.name,
        create_part_payload.name(),
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
