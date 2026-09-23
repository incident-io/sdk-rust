//! Checks the client against a local HTTP server: auth, URL building and
//! deserialisation, without needing an API key.

use incident_io::apis::{configuration::Configuration, severities_v1_api};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SEVERITIES_BODY: &str = r#"{
  "severities": [{
    "id": "01FCNDV6P870EA6S7TK1DSYDG0",
    "name": "Minor",
    "description": "Issues with low impact.",
    "rank": 1,
    "created_at": "2021-08-17T13:28:57.801578Z",
    "updated_at": "2021-08-17T13:28:57.801578Z"
  }]
}"#;

async fn server_returning(status: u16, body: &str) -> (MockServer, Configuration) {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/severities"))
        .respond_with(ResponseTemplate::new(status).set_body_raw(body, "application/json"))
        .mount(&server)
        .await;

    let mut config = Configuration::new();
    config.base_path = server.uri();
    config.bearer_access_token = Some("test-key".to_owned());

    (server, config)
}

#[tokio::test]
async fn sends_the_bearer_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/severities"))
        // The assertion: without securitySchemes in the schema the generator
        // emits no auth at all, and this header never arrives.
        .and(header("authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(SEVERITIES_BODY, "application/json"))
        .mount(&server)
        .await;

    let mut config = Configuration::new();
    config.base_path = server.uri();
    config.bearer_access_token = Some("test-key".to_owned());

    let result = severities_v1_api::severities_v1_list(&config).await;

    assert!(result.is_ok(), "{result:?}");
}

#[tokio::test]
async fn deserialises_a_response() {
    let (_server, config) = server_returning(200, SEVERITIES_BODY).await;

    let result = severities_v1_api::severities_v1_list(&config)
        .await
        .unwrap();

    assert_eq!(result.severities.len(), 1);
    assert_eq!(result.severities[0].name, "Minor");
    // Timestamps come back parsed, not as strings.
    assert_eq!(
        result.severities[0].created_at.date_naive().to_string(),
        "2021-08-17"
    );
}

#[tokio::test]
async fn an_error_status_is_an_err() {
    let body = r#"{"type":"validation_error","status":422,"request_id":"x","errors":[]}"#;
    let (_server, config) = server_returning(422, body).await;

    let result = severities_v1_api::severities_v1_list(&config).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn identifies_itself_in_the_user_agent() {
    // The generator's default names itself, so incident.io could not tell a
    // Rust SDK caller from any other generated client.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/severities"))
        .and(header(
            "user-agent",
            concat!("incident-io-sdk-rust/", env!("CARGO_PKG_VERSION")),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_raw(SEVERITIES_BODY, "application/json"))
        .mount(&server)
        .await;

    let mut config = Configuration::new();
    config.base_path = server.uri();

    assert!(severities_v1_api::severities_v1_list(&config).await.is_ok());
}
