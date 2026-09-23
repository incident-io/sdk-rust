//! Checks on what scripts/fix_generated.py reshapes.
//!
//! These are the properties that make an additive API change additive for a
//! Rust consumer. This file is an integration test, so it compiles as a
//! separate crate — which is the only place `#[non_exhaustive]` binds, and
//! therefore the only place these can be checked at all.

use incident_io::apis::incidents_v2_api::IncidentsV2ListParams;
use incident_io::models::policy_reminder_cadence_v2::Interval;

#[test]
fn a_params_struct_with_no_required_fields_takes_no_arguments() {
    let params = IncidentsV2ListParams::new().set_page_size(25);

    assert_eq!(params.page_size, Some(25));
}

#[test]
fn a_params_struct_names_its_required_fields() {
    // Required parameters are arguments, not setters, so forgetting the
    // incident ID is a compile error rather than a request to `/v2/incidents/`.
    use incident_io::apis::incidents_v2_api::IncidentsV2ShowParams;

    let params = IncidentsV2ShowParams::new("01ABC".to_owned());

    assert_eq!(params.id, "01ABC");
}

#[test]
fn setters_chain_and_unwrap() {
    // The setter takes the value, not `Some(value)`: the wrapping is an
    // artifact of the schema being rendered into Rust, not something a caller
    // should reproduce at every call site.
    let params = IncidentsV2ListParams::new()
        .set_page_size(100)
        .set_after("01ABC".to_owned());

    assert_eq!(params.page_size, Some(100));
    assert_eq!(params.after.as_deref(), Some("01ABC"));
}

#[test]
fn a_setter_unwraps_a_boxed_field_too() {
    // `Option<Box<T>>` is set from a `T`. This is the shape every nested
    // model takes, so getting it wrong would be felt everywhere.
    use incident_io::models::{IncidentsListResultV2, PaginationMetaResultWithTotalV2};

    let result = IncidentsListResultV2::new(vec![])
        .set_pagination_meta(PaginationMetaResultWithTotalV2::new(25));

    assert_eq!(result.pagination_meta.map(|meta| meta.page_size), Some(25));
}

#[test]
fn an_unknown_enum_value_deserialises_rather_than_failing() {
    // The server starts sending a new value the moment it ships. Without the
    // catch-all this fails the whole response, on every installed copy.
    let parsed: Interval = serde_json::from_str("\"hourly\"").unwrap();

    assert_eq!(parsed, Interval::Unknown("hourly".to_owned()));
}

#[test]
fn an_unknown_enum_value_round_trips_unchanged() {
    // A unit catch-all would serialize back as the literal "Unknown", so
    // reading a resource and writing it back would corrupt the field.
    let parsed: Interval = serde_json::from_str("\"hourly\"").unwrap();

    assert_eq!(serde_json::to_string(&parsed).unwrap(), "\"hourly\"");
}

#[test]
fn distinct_unknown_values_stay_distinct() {
    let one: Interval = serde_json::from_str("\"hourly\"").unwrap();
    let two: Interval = serde_json::from_str("\"fortnightly\"").unwrap();

    assert_ne!(one, two);
}

#[test]
fn known_enum_values_are_unaffected() {
    let parsed: Interval = serde_json::from_str("\"daily\"").unwrap();

    assert_eq!(parsed, Interval::Daily);
    assert_eq!(serde_json::to_string(&parsed).unwrap(), "\"daily\"");
}

/// Compiles the README's enum example, which is otherwise unchecked.
///
/// The quickstart is a doctest in `src/lib.rs`, so `cargo test` covers it. The
/// README's other blocks had no coverage and one of them named a type that
/// does not exist. Anything shown in the README belongs here or in a doctest.
#[test]
fn the_readme_enum_example_compiles() {
    use incident_io::models::incident_v2::Mode;

    fn describe(mode: &Mode) -> String {
        match mode {
            Mode::Standard => "standard".to_owned(),
            Mode::Retrospective => "retrospective".to_owned(),
            other => format!("unhandled mode: {other:?}"),
        }
    }

    assert_eq!(describe(&Mode::Standard), "standard");
    assert_eq!(
        describe(&Mode::Unknown("from_the_future".to_owned())),
        "unhandled mode: Unknown(\"from_the_future\")"
    );
}

/// The per-operation error enums are `#[non_exhaustive]` too, so documenting
/// another status code on an endpoint isn't a break. The wildcard arm is what
/// the attribute forces; without it this stops compiling.
///
/// Also compiles the README's error-handling example, down to the shape of
/// `Error::ResponseError`.
#[test]
fn matching_an_operation_error_needs_a_wildcard_arm() {
    use incident_io::apis::{incidents_v2_api::IncidentsV2ListError, Error, ResponseContent};

    fn describe(error: &Error<IncidentsV2ListError>) -> String {
        match error {
            Error::ResponseError(response) => match &response.entity {
                Some(IncidentsV2ListError::Status401(_)) => "bad API key".to_owned(),
                Some(IncidentsV2ListError::Status429(_)) => "rate limited".to_owned(),
                _ => format!("HTTP {}: {}", response.status, response.content),
            },
            other => format!("request failed: {other}"),
        }
    }

    // ResponseContent is deliberately left constructible: it comes from the
    // generator's own templates, not the schema, so it cannot grow a field
    // because the API changed. Mocking an error response needs it.
    let unknown = Error::ResponseError(ResponseContent {
        status: reqwest::StatusCode::IM_A_TEAPOT,
        content: "short and stout".to_owned(),
        entity: Some(IncidentsV2ListError::UnknownValue(serde_json::Value::Null)),
    });

    assert_eq!(describe(&unknown), "HTTP 418 I'm a teapot: short and stout");
}

/// Compiles the README's Configuration example: every field it names has to
/// exist with the type shown, and the defaults have to be what it claims.
///
/// `Configuration` is also left constructible, for the same reason as
/// `ResponseContent`, so the struct literal below is part of the assertion.
#[test]
fn the_readme_configuration_example_compiles() {
    use incident_io::apis::configuration::Configuration;
    use std::time::Duration;

    let defaults = Configuration::new();
    assert_eq!(defaults.base_path, "https://api.incident.io");
    assert!(defaults
        .user_agent
        .as_deref()
        .is_some_and(|agent| agent.starts_with("incident-io-sdk-rust/")));

    let mut config = Configuration::new();
    config.bearer_access_token = Some("my-api-key".to_owned());
    config.user_agent = Some("my-app/1.0.0".to_owned());
    config.base_path = "https://api.incident.io".to_owned();
    config.client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap();

    assert_eq!(config.user_agent.as_deref(), Some("my-app/1.0.0"));
}

/// Compiles the README's alert-ingest example. These two endpoints send no
/// `Authorization` header of their own, so the source secret has to go on the
/// client — and if the generated functions ever start reading
/// `bearer_access_token`, the README section is wrong and should go.
#[test]
fn the_readme_alert_ingest_example_compiles() {
    use incident_io::apis::configuration::Configuration;
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

    let alert_source_secret = "secret-from-the-alert-source";

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {alert_source_secret}")).unwrap(),
    );

    let mut config = Configuration::new();
    config.client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    assert!(config.bearer_access_token.is_none());
}

/// Compiles the README's request-payload example, including the nesting: a
/// payload built with `new()` plus setters, handed to a params struct that
/// requires it.
#[test]
fn the_readme_payload_example_compiles() {
    use incident_io::apis::incidents_v2_api::IncidentsV2CreateParams;
    use incident_io::models::incidents_create_payload_v2::Visibility;
    use incident_io::models::IncidentsCreatePayloadV2;

    let idempotency_key = "once-only".to_owned();

    let body = IncidentsCreatePayloadV2::new(idempotency_key, Visibility::Public)
        .set_name("Checkout is down".to_owned());

    let params = IncidentsV2CreateParams::new(body);

    assert_eq!(
        params.incidents_create_payload_v2.name.as_deref(),
        Some("Checkout is down")
    );
}

/// Compiles the README's pagination loop. `set_after` is conditional, so the
/// params value has to be rebindable rather than a single chained expression
/// — which is the one place the setter style reads worse than a literal, and
/// worth keeping honest.
#[test]
fn the_readme_pagination_example_compiles() {
    use incident_io::apis::incidents_v2_api::IncidentsV2ListParams;

    let after: Option<String> = Some("01ABC".to_owned());

    let mut params = IncidentsV2ListParams::new().set_page_size(100);
    if let Some(cursor) = &after {
        params = params.set_after(cursor.clone());
    }

    assert_eq!(params.page_size, Some(100));
    assert_eq!(params.after.as_deref(), Some("01ABC"));
}
