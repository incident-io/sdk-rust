# Working on this repo

Everything under `src/apis/` and `src/models/` is generated. Don't edit it: the
next release overwrites it. Changes to the API surface come from the upstream
OpenAPI schema; changes to the *shape* of the generated code come from
`scripts/fix_generated.py`.

Hand-written and safe to edit: `src/lib.rs`, `tests/`, `Cargo.toml`, the
`Makefile`, `scripts/`, `.github/`, and the docs.

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
  published baseline, so it says nothing before the first release.

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

Measured against sdk-go's 124 committed schemas, 30 of them add an optional
request property or parameter — about one release in four. Without this the
release would halt on every one of those and need a human to cut a major
version. Its module docstring is the full argument; read it before changing any
of the passes.

The cost lands on callers, who can no longer write a struct literal for these
types. `aws-sdk-s3` and `google-cloud-storage` make the same trade; every Rust
API client that still takes struct literals is `0.x`, where a minor release is
allowed to break you.

## First release checklist

The crate does not exist on crates.io yet, and several things are switched off
until it does. In order:

1. **Publish once by hand.** Trusted publishing cannot create a crate, only
   publish to one that exists. Set `version` in `Cargo.toml`, run `make test`,
   then `cargo publish` with a token from crates.io.
2. **Tag it**, so `Decide next version` has a baseline:
   `git tag v1.0.0 && git push origin v1.0.0`.
3. **Register trusted publishing** on crates.io against this repository **and
   the workflow filename `sync.yml`**. Renaming that file silently breaks
   publishing.
4. **Uncomment the `schedule:` block** at the top of
   `.github/workflows/sync.yml`. Nothing else enables the loop, and nothing
   checks that you did: until this happens the repo looks healthy and publishes
   nothing.
5. **Run the workflow once manually** with `dry_run: true` and confirm it gets
   as far as reporting a version.

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
