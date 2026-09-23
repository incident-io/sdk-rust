# Working on this repo

Everything under `src/apis/` and `src/models/` is generated. Don't edit it: the
next release overwrites it. Changes to the API surface come from the upstream
OpenAPI schema; changes to the *shape* of the generated code come from
`scripts/fix_generated.py`.

Hand-written and safe to edit: `src/lib.rs`, `tests/`, `Cargo.toml`, the
`Makefile`, `scripts/`, `.github/`, `README.md` and this file.

`docs/` is not: openapi-generator writes ~1200 markdown files there. It is
gitignored and removed by `make clean`; docs.rs builds from the source instead.

## Prerequisites

| Tool | Needed by | Notes |
| --- | --- | --- |
| Rust + `rustup` | everything | `rust-toolchain.toml` pins the version; rustup installs it for you |
| the MSRV toolchain | `make msrv` | `rustup toolchain install $(grep -m1 '^MSRV' Makefile \| cut -d= -f2)`. rustup no longer auto-installs for an explicit `cargo +<version>`, so without this the target fails with "no such command" |
| Java 11 or later | `make generate`, `make template-drift` | openapi-generator is a jar; the Makefile downloads it to `/tmp` |
| Python 3 | `make generate` | runs `scripts/fix_generated.py` |
| network | `make verify`, `make oasdiff` | `cargo publish --dry-run` contacts crates.io, and `make oasdiff` fetches the live schema |

## Targets

`make help` lists them. The ones that matter:

- `make generate` — regenerate from the **committed** `openapi.json`, run the
  post-generation pass, format. Start here after changing anything in
  `scripts/`.
- `make test` — build, clippy, `cargo doc`, a packaging dry-run, then the
  tests. This is what CI runs.
- `make msrv` — build on the declared minimum Rust version. CI runs it on every
  pull request *and* in the release, because the floor climbs on its own as
  transitive dependencies raise theirs.
- `make template-drift` — fail if the generator's templates changed under the
  anchors `scripts/fix_generated.py` matches on.
- `make oasdiff` — the schema gate that stops a release, runnable by hand.
- `make semver-checks` — the Rust API gate that stops a release. Needs a
  published baseline, so before the first release it exits non-zero with
  "not found in registry". That is expected; the release workflow probes
  crates.io and skips the gate instead.

## How a release happens

`.github/workflows/sync.yml`, hourly. When the live schema differs from the
committed one it regenerates, verifies, bumps the **minor** version, commits,
tags, and publishes to crates.io. No human unless a gate trips.

Two gates stop it. `oasdiff` compares the schemas and `cargo-semver-checks`
compares the Rust API against the last published crate; either one reporting a
break halts the run and files an issue. A halted run does **not** commit the new
schema, so every later run sees the same diff and halts the same way until
someone acts — which is deliberate, and why the issue is deduped.

To release a breaking change, run the workflow from the Actions tab with
**bump: major** and **acknowledge_breaking: true**. Both are required together.

### Why the generated code is shaped the way it is

Our API compatibility policy treats adding a response property, a request
property, an enum value or an optional parameter as backwards-compatible, and
those ship without a human. In Rust each one is a hard break on ordinary
generated types, so `scripts/fix_generated.py` reshapes the output to absorb
them: `#[non_exhaustive]` on every schema-derived struct and enum, an
`Unknown(String)` catch-all on the enums, and a generated `new()` plus `set_*`
methods so the marked structs can still be built.

Measured against sdk-go's 124 committed schema diffs, 29 of them add an
optional request property or parameter — about one release in four — and 13
add a required property to an existing response model, which is why those
models get a zero-argument `new()`. Without this the release would halt on
every one of those and need a human to cut a major version. The module
docstring is the full argument; read it before changing any of the passes.

The cost lands on callers, who can no longer write a struct literal for these
types. `aws-sdk-s3` and `google-cloud-storage`, the two Rust clients generated
from a spec and released continuously at a stable major version, both make the
same trade.

## Keeping the loop alive

Everything that reports a problem here is a `failure()` hook, and a loop that
never runs never fails. Two ways it can go quiet:

- GitHub **disables a scheduled workflow after 60 days** with no repository
  activity, and emails only whoever last edited the cron. This repo's activity
  is its own release commits, which stop exactly when the schema stops
  changing — so the disable lands when nothing else would show it.
- Someone reverts or never enables the cron.

Each successful run writes a line to its job summary saying what it decided, so
"when did this last sync?" is answerable from the Actions run list. If the run
list is empty for a week, the loop is off, not quiet.

## Upgrading openapi-generator

`OPENAPI_GENERATOR_VERSION` in the `Makefile` is pinned deliberately. The
project ships no patch releases, so every available upgrade is a minor, which
its own policy says may change template-bound variables — and those variables
are what `scripts/fix_generated.py` anchors on.

Bumping it is a read-the-diff operation:

1. Change the version, run `make template-drift`. It will fail and print the
   diff against `templates/pristine/`.
2. Read the diff. Decide whether each pass in `scripts/fix_generated.py` still
   matches.
3. Copy the new templates over `templates/pristine/`, run `make generate`, and
   check the pass counts did not collapse.
4. `make test`.
