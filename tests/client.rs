//! Checks the client against a local HTTP server: auth, URL building and
//! deserialisation, without needing an API key.

use incident_io::apis::configuration::Configuration;
use incident_io::apis::severities_v1_api::{self, SeveritiesV1ListParams};
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

    let result =
        severities_v1_api::severities_v1_list(&config, SeveritiesV1ListParams::new()).await;

    assert!(result.is_ok(), "{result:?}");
}

#[tokio::test]
async fn deserialises_a_response() {
    let (_server, config) = server_returning(200, SEVERITIES_BODY).await;

    let result = severities_v1_api::severities_v1_list(&config, SeveritiesV1ListParams::new())
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

    let result =
        severities_v1_api::severities_v1_list(&config, SeveritiesV1ListParams::new()).await;

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

    assert!(
        severities_v1_api::severities_v1_list(&config, SeveritiesV1ListParams::new())
            .await
            .is_ok()
    );
}

/// One-level filters go on the wire as `created_at[gte]=2024-05-01`.
///
/// The generator sends the whole object as one JSON-encoded value, which the
/// API ignores — the request succeeds and returns unfiltered results, so
/// nothing short of reading the query string catches it.
#[tokio::test]
async fn object_filters_use_the_bracket_form() {
    use incident_io::apis::incidents_v2_api::{self, IncidentsV2ListParams};
    use std::collections::HashMap;

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/incidents"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(r#"{"incidents":[]}"#, "application/json"),
        )
        .mount(&server)
        .await;

    let mut config = Configuration::new();
    config.base_path = server.uri();

    let mut created_at = HashMap::new();
    created_at.insert("gte".to_owned(), vec!["2024-05-01".to_owned()]);

    let params = IncidentsV2ListParams::new().set_created_at(created_at);
    incidents_v2_api::incidents_v2_list(&config, params)
        .await
        .unwrap();

    let request = &server.received_requests().await.unwrap()[0];
    let query: Vec<(String, String)> = request
        .url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    // url::Url decodes the percent-encoded brackets, as Go's net/url does.
    assert!(
        query.contains(&("created_at[gte]".to_owned(), "2024-05-01".to_owned())),
        "expected created_at[gte]=2024-05-01, got {query:?}"
    );
    // Not the JSON blob the generator would have sent.
    assert!(!query.iter().any(|(k, _)| k == "created_at"), "{query:?}");
}

/// Two-level filters nest: `custom_field[<id>][one_of]=<value>`.
///
/// This is the case a one-level flatten gets wrong — it stringifies the inner
/// map, and the one-level filters keep working, so the bug looks fixed.
/// Multiple values repeat the key rather than indexing it.
#[tokio::test]
async fn nested_object_filters_keep_both_levels() {
    use incident_io::apis::incidents_v2_api::{self, IncidentsV2ListParams};
    use std::collections::HashMap;

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/incidents"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(r#"{"incidents":[]}"#, "application/json"),
        )
        .mount(&server)
        .await;

    let mut config = Configuration::new();
    config.base_path = server.uri();

    let mut operators = HashMap::new();
    operators.insert(
        "one_of".to_owned(),
        vec!["01ABC".to_owned(), "01DEF".to_owned()],
    );
    let mut custom_field = HashMap::new();
    custom_field.insert("01FIELD".to_owned(), operators);

    let params = IncidentsV2ListParams::new().set_custom_field(custom_field);
    incidents_v2_api::incidents_v2_list(&config, params)
        .await
        .unwrap();

    let request = &server.received_requests().await.unwrap()[0];
    let query: Vec<(String, String)> = request
        .url
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();

    let key = "custom_field[01FIELD][one_of]".to_owned();
    assert!(
        query.contains(&(key.clone(), "01ABC".to_owned())),
        "{query:?}"
    );
    assert!(query.contains(&(key, "01DEF".to_owned())), "{query:?}");
    // Repeated, not indexed.
    assert!(
        !query.iter().any(|(k, _)| k.contains("[0]")),
        "values were indexed rather than repeated: {query:?}"
    );
}
