//! Rust client for the [incident.io](https://incident.io) API.
//!
//! Everything under [`apis`] and [`models`] is generated from our published
//! OpenAPI schema, so it tracks the live API: a function for every endpoint,
//! and a type for every request and response.
//!
//! # Quickstart
//!
//! ```no_run
//! use incident_io::apis::{configuration::Configuration, incidents_v2_api};
//!
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! let mut config = Configuration::new();
//! config.bearer_access_token = Some("my-api-key".to_owned());
//!
//! let params = incidents_v2_api::IncidentsV2ListParams::new().set_page_size(25);
//! let result = incidents_v2_api::incidents_v2_list(&config, params).await?;
//!
//! for incident in result.incidents {
//!     println!("{} {}", incident.reference, incident.name);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Building a request
//!
//! Every generated type is `#[non_exhaustive]`, so that adding a field stays a
//! backwards-compatible change. Struct literals and `..Default::default()`
//! therefore don't compile from outside this crate; use `new()`, which takes
//! the required fields, and the chainable `set_*` methods for the rest:
//!
//! ```
//! use incident_io::apis::incidents_v2_api::IncidentsV2ListParams;
//!
//! let params = IncidentsV2ListParams::new()
//!     .set_page_size(100)
//!     .set_sort_by("created_at_newest_first");
//! ```
//!
//! A setter takes the value rather than `Some(value)`. Reading a field is
//! unchanged — they are all public.
//!
//! `new()` takes the required fields on a params struct and on a request
//! payload. On a **response** type it takes none: the API adds required
//! properties to responses regularly, and a constructor that named them would
//! break you every time it did.
//!
//! [`apis::configuration::Configuration`], [`apis::configuration::ApiKey`],
//! [`apis::ResponseContent`] and [`apis::Error`] are `#[non_exhaustive]` as
//! well. An API change cannot add a field to those, but a generator upgrade
//! can, so they are marked for the same reason and built the same way.
//! [`apis::Error`] needs a wildcard arm when you match it.
//!
//! `Default::default()` is *not* blocked by `#[non_exhaustive]`, and every
//! model derives it. On a type with required fields it hands back empty
//! strings, which the API rejects with a 422 — use `new()` instead.
//!
//! # Unknown enum values
//!
//! We add values to enums as a backwards-compatible change, so every generated
//! enum carries an `Unknown(String)` variant holding the value verbatim. Match
//! the variants you care about and treat `Unknown` as unrecognised; it
//! serializes back as the original string, so reading a resource and writing
//! it again does not corrupt a field this build does not know about.
//!
//! # TLS
//!
//! `rustls` by default. For OpenSSL instead:
//!
//! ```toml
//! incident-io = { version = "1", default-features = false, features = ["native-tls"] }
//! ```
//!
//! Cargo features are additive across a dependency graph, so another crate
//! asking for `native-tls` brings it back in alongside rustls. Pick the
//! backend explicitly on the `reqwest::Client` if you need to be certain.
//!
//! Turning both off is a compile error rather than a client that cannot
//! reach the API.

#![allow(unused_imports)]
#![allow(clippy::too_many_arguments)]
// Every doc comment below this line is prose from the OpenAPI schema, so it
// contains curl examples: bare URLs, and query filters like
// `status[one_of]=firing` that rustdoc reads as links to Rust items. Without
// these allows the schema's own examples would have to be mangled to keep
// rustdoc quiet.
//
// The cost is real, so it is written down: the generator also emits ~608
// genuine intra-doc links between operations, and `broken_intra_doc_links`
// is the lint that would catch one of those going stale. Measured at the time
// of writing with `RUSTDOCFLAGS="--force-warn rustdoc::broken_intra_doc_links"`
// — 53 warnings, every one of them the schema-prose kind, none from a real
// link. Re-run that if the generated links ever start looking wrong.
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::broken_intra_doc_links)]

// Both TLS features are optional and `rustls` is only a default, so
// `default-features = false` with neither of them selected compiles clean and
// produces a client that cannot open an HTTPS connection to api.incident.io —
// discovered at runtime, on the first request. Say it at compile time instead.
#[cfg(not(any(feature = "rustls", feature = "native-tls")))]
compile_error!(
    "incident-io needs a TLS backend. You set default-features = false without \
     selecting one: add features = [\"rustls\"] (the default) or \
     features = [\"native-tls\"]."
);

pub mod apis;
pub mod models;
