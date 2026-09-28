//! Error handling, against a real HTTP response.
//!
//! The shape tests build an `Error` by hand, which proves the match arms
//! compile and nothing else. Both bugs covered here were invisible to that:
//! one is in how the entity is chosen from the response, the other in a
//! `Debug` impl. Neither is reachable without going through the client.

use incident_io::apis::configuration::Configuration;
use incident_io::apis::incidents_v2_api::{self, IncidentsV2ListError, IncidentsV2ListParams};
use incident_io::apis::Error;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn error_body(status: u16, kind: &str) -> String {
    format!(
        r#"{{"type":"{kind}","status":{status},"request_id":"01ABC",
            "errors":[{{"code":"c","message":"m"}}]}}"#
    )
}

async fn list_returning(status: u16, kind: &str) -> Error<IncidentsV2ListError> {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v2/incidents"))
        .respond_with(
            ResponseTemplate::new(status)
                .set_body_raw(error_body(status, kind), "application/json"),
        )
        .mount(&server)
        .await;

    let mut config = Configuration::new();
    config.base_path = server.uri();

    incidents_v2_api::incidents_v2_list(&config, IncidentsV2ListParams::new())
        .await
        .expect_err("a non-2xx status should be an Err")
}

/// Every variant of a `*Error` enum holds the same `models::ErrorResponse`
/// and the enum is `#[serde(untagged)]`, so deserializing the body returns
/// whichever variant is declared first — always the lowest status code. A 401
/// arrived as `Status400`, which made the README's `Status401` and
/// `Status429` arms unreachable on every endpoint.
#[tokio::test]
async fn the_error_variant_matches_the_http_status() {
    for (status, kind) in [
        (401u16, "authentication_error"),
        (429, "too_many_requests"),
        (500, "internal_server_error"),
    ] {
        let error = list_returning(status, kind).await;
        let Error::ResponseError(response) = error else {
            panic!("expected a ResponseError for {status}");
        };

        assert_eq!(response.status.as_u16(), status);
        let matched = matches!(
            (&response.entity, status),
            (Some(IncidentsV2ListError::Status401(_)), 401)
                | (Some(IncidentsV2ListError::Status429(_)), 429)
                | (Some(IncidentsV2ListError::Status500(_)), 500)
        );
        assert!(matched, "HTTP {status} produced {:?}", response.entity);
    }
}

/// A status the endpoint does not document falls through to `UnknownValue`
/// rather than being forced into the nearest variant.
#[tokio::test]
async fn an_undocumented_status_is_unknown() {
    let error = list_returning(418, "teapot").await;
    let Error::ResponseError(response) = error else {
        panic!("expected a ResponseError");
    };

    assert!(
        matches!(response.entity, Some(IncidentsV2ListError::UnknownValue(_))),
        "HTTP 418 produced {:?}",
        response.entity
    );
}

/// `Configuration` derives `Clone` but not `Debug`, because the derived one
/// printed `bearer_access_token` in full — so `tracing::debug!(?config)` or a
/// panic message would put the API key in logs.
#[test]
fn debug_does_not_print_the_api_key() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("inc_live_SECRET_VALUE".to_owned());

    let rendered = format!("{config:?}");

    assert!(!rendered.contains("SECRET_VALUE"), "{rendered}");
    assert!(rendered.contains("***"), "{rendered}");
    // The harmless fields still show, so the impl stays useful for debugging.
    assert!(rendered.contains("https://api.incident.io"), "{rendered}");
}
