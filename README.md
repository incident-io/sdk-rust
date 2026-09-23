# incident.io Rust SDK

[![crates.io](https://img.shields.io/crates/v/incident-io)](https://crates.io/crates/incident-io)
[![docs.rs](https://img.shields.io/docsrs/incident-io)](https://docs.rs/incident-io)

The official Rust client for the [incident.io](https://incident.io)
[public API](https://api-docs.incident.io/).

It is generated automatically from our published OpenAPI schema, so it always
tracks the live API — there is a function for every endpoint, and a type for
every request and response.

## Install

```bash
cargo add incident-io
```

Requires Rust 1.88 or later.

## Quickstart

Create an API key in your incident.io dashboard under **Settings → API keys**,
then:

```rust
use incident_io::apis::{configuration::Configuration, incidents_v2_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("my-api-key".to_owned());

    let params = incidents_v2_api::IncidentsV2ListParams::new().set_page_size(25);
    let result = incidents_v2_api::incidents_v2_list(&config, params).await?;

    for incident in result.incidents {
        println!("{} {}", incident.reference, incident.name);
    }

    Ok(())
}
```

Every endpoint is an async function taking a `&Configuration` and a params
struct.

`new()` takes the endpoint's **required** parameters, positionally. Optional
ones are chainable `set_*` methods, which take the value rather than
`Some(value)`:

```rust
// No required parameters.
let params = incidents_v2_api::IncidentsV2ListParams::new()
    .set_page_size(25)
    .set_sort_by("newest".to_owned());

// One required path parameter.
let params = incidents_v2_api::IncidentsV2ShowParams::new("01ABC...".to_owned());
```

Request payloads work the same way, and nest:

```rust
use incident_io::models::incidents_create_payload_v2::Visibility;
use incident_io::models::IncidentsCreatePayloadV2;

let body = IncidentsCreatePayloadV2::new(idempotency_key, Visibility::Public)
    .set_name("Checkout is down".to_owned());

let params = incidents_v2_api::IncidentsV2CreateParams::new(body);
```

Forgetting a required parameter is a compile error. Adding an optional one
never is — see [Adding fields](#adding-fields).

## Pagination

List endpoints are cursor-paginated. Read the next cursor from
`pagination_meta.after` and pass it back:

```rust
let mut after: Option<String> = None;

loop {
    let mut params = incidents_v2_api::IncidentsV2ListParams::new().set_page_size(100);
    if let Some(cursor) = &after {
        params = params.set_after(cursor.clone());
    }

    let page = incidents_v2_api::incidents_v2_list(&config, params).await?;

    for incident in &page.incidents {
        println!("{} {}", incident.reference, incident.name);
    }

    match page.pagination_meta.and_then(|meta| meta.after) {
        Some(cursor) => after = Some(cursor),
        None => break,
    }
}
```

## Unknown enum values

We add values to enums as a backwards-compatible change, so every enum
generated from a schema value carries an `Unknown(String)` variant and is
marked `#[non_exhaustive]`. Match the variants you care about and let the
wildcard handle the rest:

```rust
use incident_io::models::incident_v2::Mode;

match &incident.mode {
    Mode::Standard => println!("standard"),
    Mode::Retrospective => println!("retrospective"),
    other => println!("unhandled mode: {other:?}"),
}
```

The wildcard arm is mandatory, because these enums are `#[non_exhaustive]` —
that is what makes a new value a non-breaking change rather than a compile
error in your code.

The variant holds the value verbatim and serializes back to it unchanged, so
reading a resource and writing it back doesn't discard a field this build
doesn't recognise. The cost is that these enums are not `Copy`: match on a
reference, or clone.

## Adding fields

We add response properties, request properties and optional parameters as
backwards-compatible changes. On an ordinary Rust struct each one is a hard
break, so **every generated type is `#[non_exhaustive]`** — responses, request
payloads and params structs alike.

That is why there are no struct literals in this README. `Foo { .. }` and
`..Default::default()` don't compile from outside the crate for these types;
`Foo::new(required...)` and the `set_*` methods are the way in, and they keep
working when the API grows a field.

```rust
use incident_io::models::IncidentsListResultV2;

// Building one, for a fixture or a mock.
let result = IncidentsListResultV2::new(vec![]);

// Reading one is unchanged: the fields are public.
for incident in &result.incidents {
    println!("{}", incident.reference);
}
```

`Configuration` and `ResponseContent` are the exceptions, and stay ordinary
structs. They come from the code generator's own templates rather than from the
API schema, so they can't grow a field because the API changed.

This is the same trade `aws-sdk-s3` and `google-cloud-storage` make. The
clients that still take struct literals are all `0.x`, where SemVer allows a
minor release to break you.

## Errors

Every endpoint returns `Result<T, Error<SomethingError>>`, where the inner enum
has a variant per documented status code plus an
`UnknownValue(serde_json::Value)` for anything else. Documenting another status
code is a backwards-compatible change, so those enums are `#[non_exhaustive]`
too and a wildcard arm is required:

```rust
use incident_io::apis::{Error, incidents_v2_api::IncidentsV2ListError};

match incidents_v2_api::incidents_v2_list(&config, params).await {
    Ok(result) => println!("{} incidents", result.incidents.len()),
    Err(Error::ResponseError(response)) => match response.entity {
        Some(IncidentsV2ListError::Status401(_)) => println!("bad API key"),
        Some(IncidentsV2ListError::Status429(_)) => println!("rate limited"),
        _ => println!("HTTP {}: {}", response.status, response.content),
    },
    Err(other) => println!("request failed: {other}"),
}
```

## Configuration

`Configuration::new()` gives you the defaults; every field is public, so change
what you need:

```rust
use std::time::Duration;
use incident_io::apis::configuration::Configuration;

let mut config = Configuration::new();
config.bearer_access_token = Some("my-api-key".to_owned());

// Identify your integration. Defaults to `incident-io-sdk-rust/<version>`.
config.user_agent = Some("my-app/1.0.0".to_owned());

// Defaults to https://api.incident.io.
config.base_path = "https://api.incident.io".to_owned();

// Timeouts, proxies, connection pooling and TLS all live on the client.
config.client = reqwest::Client::builder()
    .timeout(Duration::from_secs(30))
    .build()?;
```

`bearer_access_token` is the one you need; `basic_auth`, `oauth_access_token`
and `api_key` are emitted by the generator and unused by this API.

### Retries

The client makes a **single attempt** per request and does not retry. We
deliberately ship no retry layer: the usual crate for it, `reqwest-middleware`,
would land in your dependency tree and change `Configuration`'s public shape
whether you wanted it or not.

The API rate-limits at 1200 requests/minute per key and answers a `429` with a
`Retry-After` header in seconds. Prefer it over `X-RateLimit-Reset`: it is a
duration, so it does not depend on your clock, and it says when a single
request will succeed rather than when your whole allowance is back. Retry on
`429` and `5xx`, and back off exponentially otherwise.

### Endpoints that don't take an API key

Four generated functions send no `Authorization` header, because the schema
marks them as needing no API key.

Two are public and need nothing: `utilities_v1_ip_ranges` and
`utilities_v1_open_apiv3`.

The other two authenticate with the secret from the alert source or heartbeat
you're posting to, not with an API key: `alert_events_v2_create_http` and
`heartbeat_v2_ping`. `bearer_access_token` is ignored on both, so set the
secret on the client:

```rust
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

let mut headers = HeaderMap::new();
headers.insert(
    AUTHORIZATION,
    HeaderValue::from_str(&format!("Bearer {alert_source_secret}"))?,
);

let mut config = Configuration::new();
config.client = reqwest::Client::builder()
    .default_headers(headers)
    .build()?;
```

Use a separate `Configuration` for these, so the source secret isn't sent on
calls that expect an API key.

There are two heartbeat functions with the same documentation, because the
schema documents both verbs on the same path: `heartbeat_v2_ping` is the POST
and takes the source secret as above, `heartbeat_v2_ping1` is the GET and takes
an ordinary API key.

### Deprecated endpoints

Deprecated endpoints stay available and carry `#[deprecated]`, so rustc warns at
the call site. Don't assume a `v2` endpoint is current: 38 operations are
deprecated today, and they span Catalog V2 (12), Follow-ups V2 (6), Actions V2
(5) and Actions V1 (2), as well as Custom Fields V1 (5), Incident Roles V1 (5)
and Incidents V1 (3). Let the compiler tell you rather than going by the version
in the name.

The release fails if the schema marks an endpoint deprecated and the generated
code doesn't, so the warnings track the API rather than this list.

## TLS

`rustls` by default. For OpenSSL instead:

```toml
incident-io = { version = "1", default-features = false, features = ["native-tls"] }
```

Cargo features are additive across a dependency graph, so another crate asking
for `native-tls` brings it back in alongside rustls. Set the backend on the
`reqwest::Client` explicitly if you need to be certain which one you get.

Turning both off is a compile error, rather than a client that cannot reach the
API.

We depend on reqwest with its own default features off, so the client is
**HTTP/1.1 only** and ignores **macOS and Windows system proxy settings**
(`HTTP_PROXY` and friends still work). If you need either, build your own
`reqwest::Client` with the features you want and assign it to
`config.client`.

## Versioning

Releases are cut automatically. A job checks the published API schema hourly,
and when it has changed, regenerates this crate, runs the tests, and publishes
a new **minor** version.

Two gates stand in front of that. [oasdiff](https://github.com/oasdiff/oasdiff)
compares the old and new schemas, and
[cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks)
compares the Rust API against the last published crate. If either reports a
breaking change the release stops and a human decides what to do, so a break is
never published as a minor version.

Patch versions are only cut by hand, for a fix to this crate that isn't a
schema change.

## Support

Found a bug or missing something? Please
[open an issue](https://github.com/incident-io/sdk-rust/issues). For questions
about the API itself, see the [API docs](https://api-docs.incident.io/).

Note that everything under `src/apis/` and `src/models/` is generated — please
don't send PRs editing it directly; changes there come from the upstream
schema. See [CONTRIBUTING.md](./CONTRIBUTING.md) if you want to work on the
repo itself.

One naming quirk comes from the generator: an acronym at the start of an
operation name is split, so the API keys endpoints are `a_pi_keys_v1_list`,
`a_pi_keys_v1_create` and so on, in the `api_keys_v1_api` module. The module
name is the searchable one.

## License

MIT — see [LICENSE](./LICENSE).

This SDK's generated code is produced by
[openapi-generator](https://github.com/OpenAPITools/openapi-generator), which
is licensed under Apache 2.0.
