# Pinned deliberately. openapi-generator ships no patch releases and no major
# since 2023 — every upgrade available is a minor, which is the tier its own
# policy says may change template-bound variables. Bumping this is a
# human-reads-the-diff operation, never automatic.
OPENAPI_GENERATOR_VERSION := 7.25.0

# The version we promise to compile on. Our own code needs far less; the floor
# comes from Unicode crates that reqwest pulls in through url, and it climbs on
# its own as those update. See the msrv target.
MSRV := 1.88

# Pinned for the same reasons the generator is: these decide whether we
# publish. Keep in step with the env block in .github/workflows/sync.yml.
OASDIFF_VERSION      := 1.32.1
SEMVER_CHECKS_VERSION := 0.50.0
# Versioned, like the generator jar: an unversioned path would keep serving a
# stale binary after OASDIFF_VERSION is bumped.
OASDIFF              := /tmp/oasdiff-$(OASDIFF_VERSION)
# oasdiff ships one universal darwin build and per-arch linux builds.
OASDIFF_OS           := $(shell uname -s | tr 'A-Z' 'a-z')
OASDIFF_PLATFORM     := $(if $(filter darwin,$(OASDIFF_OS)),darwin_all,$(OASDIFF_OS)_$(shell uname -m | sed 's/x86_64/amd64/;s/aarch64/arm64/'))

CRATE      := incident-io
SCHEMA_URL := https://api.incident.io/v1/openapiV3.json
GENERATOR  := /tmp/openapi-generator-cli-$(OPENAPI_GENERATOR_VERSION).jar

.DEFAULT_GOAL := help
.PHONY: help fetch generate verify test msrv template-drift oasdiff semver-checks clean

help:
	@grep -E '^[a-z-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-16s\033[0m %s\n", $$1, $$2}'

# -L because without it a redirect is a silent success writing zero bytes,
# which parses as an empty schema. OUT lets the release workflow fetch to a
# scratch path so it still has the previous schema to diff against.
OUT ?= openapi.json

fetch: ## Fetch the live schema (OUT= to write elsewhere)
	curl -sfSL $(SCHEMA_URL) -o $(OUT)

$(GENERATOR):
	curl -sfSL -o $@ \
		https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/$(OPENAPI_GENERATOR_VERSION)/openapi-generator-cli-$(OPENAPI_GENERATOR_VERSION).jar

# .openapi-generator-ignore keeps the generator out of Cargo.toml, README and
# the rest: the rust generator has no --meta none, and it would otherwise
# overwrite the crate metadata, the MSRV and the release version on every run.
generate: openapi.json $(GENERATOR) ## Regenerate the client from the committed schema
	# Cleared first: the generator only writes, never deletes, so an endpoint or
	# model removed upstream would otherwise leave a stale file behind — dropped
	# from mod.rs, invisible to the drift check, committed forever. Not `src`
	# itself, because lib.rs is hand-written.
	rm -rf src/apis src/models
	java -jar $(GENERATOR) generate \
		--input-spec openapi.json \
		--generator-name rust \
		--output . \
		--additional-properties=packageName=$(CRATE),reqwestDefaultFeatures=rustls,useSingleRequestParameter=true,useSerdePathToError=true \
		--global-property=skipFormModel=false \
		> /tmp/openapi-generator.log 2>&1 || (tail -40 /tmp/openapi-generator.log && exit 1)
	python3 scripts/fix_generated.py src openapi.json
	cargo fmt
	# Again, over the formatted tree, and fail if anything moves.
	#
	# The pass parses generated Rust with regexes, and cargo fmt rewrites the
	# shapes it matches on: it wraps a long field type onto its own line and a
	# long #[serde(...)] across several. Running before cargo fmt hides every
	# bug of that kind, and two of them got that far. Running again after is
	# the direct test, and it costs a second.
	# Against a snapshot, not against git: src/ is regenerated wholesale and is
	# untracked on a fresh clone, so `git diff` would compare nothing and pass.
	@rm -rf /tmp/sdk-rust-fmt-check && cp -R src /tmp/sdk-rust-fmt-check
	@python3 scripts/fix_generated.py src openapi.json > /dev/null
	@diff -rq /tmp/sdk-rust-fmt-check src || \
		(echo "" && \
		 echo "scripts/fix_generated.py is not idempotent over formatted output." && \
		 echo "It parses what cargo fmt just rewrote — most likely a wrapped field" && \
		 echo "type or a wrapped attribute. See strip_attributes and struct_fields." && \
		 exit 1)
	@rm -rf /tmp/sdk-rust-fmt-check

verify: ## Build, lint, check docs and the package
	cargo build --all-targets
	# An allow-list rather than -D warnings: the generator emits ~837 lints of
	# its own, and -D warnings over 76k lines of machine-written code is a bet
	# that no future clippy release ever objects to any of it.
	cargo clippy --all-targets -- \
		-D warnings \
		-A clippy::needless_return \
		-A clippy::derivable_impls \
		-A clippy::into_iter_on_ref \
		-A clippy::empty_docs \
		-A clippy::to_string_in_format_args \
		-A clippy::too_many_arguments
	# The README and lib.rs both tell consumers they can swap the TLS backend.
	# Nothing else builds that combination, so a reqwest minor that breaks it
	# would be found by a consumer rather than by us. --lib because the tests
	# and the doctest don't depend on the backend.
	cargo build --lib --no-default-features --features native-tls
	# docs.rs builds on its own nightly, never retries a failure and tells
	# nobody, so a broken build is only visible as a crate with no docs.
	cargo doc --no-deps
	# --allow-dirty because verification runs before the release commit, so the
	# tree always has the regenerated client in it.
	#
	# Redirect and then cat, rather than piping through tee: make runs each line
	# under /bin/sh, which is dash on the CI image and has no pipefail, and a
	# pipeline's status is the last command's — so a failed publish check would
	# be masked by tee exiting 0 and the release would proceed on a crate that
	# does not package.
	cargo publish --dry-run --allow-dirty > /tmp/publish-dry-run.log 2>&1 || (cat /tmp/publish-dry-run.log && exit 1)
	@cat /tmp/publish-dry-run.log
	# --dry-run downgrades "version already exists" to a warning and exits 0, so
	# without this a re-run would sail through and fail at the real upload.
	#
	# Release path only: on a pull request Cargo.toml still holds the version the
	# last release committed, which is published by definition, so checking there
	# would redden every PR.
	@if [ -n "$$RELEASING" ]; then \
		! grep -q "already exists on crates.io index" /tmp/publish-dry-run.log \
			|| (echo "This version is already on crates.io" && exit 1); \
	fi

test: verify ## Everything verify does, plus the tests
	cargo test

# The declared MSRV is a promise nothing else checks: ubuntu-latest runs current
# stable, so a green build there says nothing about it. Run this in the release
# workflow too, not only on pull requests — otherwise the first sign that a
# transitive dependency raised the floor is a red run after the crate is
# already published, and crates.io versions cannot be replaced.
msrv: ## Build on the declared minimum Rust version
	# The number is written in three places: here, Cargo.toml's rust-version
	# (the promise a consumer's cargo enforces) and the README. Only this one
	# is checked by building, so the other two can drift into a promise nothing
	# tests. Assert they agree before spending the build.
	@grep -q '^rust-version = "$(MSRV)"' Cargo.toml || 		(echo "Cargo.toml rust-version disagrees with MSRV=$(MSRV) in the Makefile" && exit 1)
	@grep -q 'Requires Rust $(MSRV) or later' README.md || 		(echo "README.md disagrees with MSRV=$(MSRV) in the Makefile" && exit 1)
	# +$(MSRV) overrides rust-toolchain.toml, so this needs rustup.
	cargo +$(MSRV) build

# We deliberately do not fork the generator's templates — scripts/fix_generated.py
# explains why — so templates/pristine/ holds unmodified upstream copies, used
# for nothing but this check. The generator is never invoked with -t.
#
# The check exists because rewriting generated text anchors on what the
# templates emit. An upstream edit to a template shows up as a pass matching
# nothing, which stops the release; an upstream *rename or split* of the file
# would not show up at all. So assert both: the template still exists under the
# same name, and it still says what the anchors were written against.
template-drift: $(GENERATOR) ## Fail if the generator's templates moved under us
	rm -rf /tmp/upstream-templates
	java -jar $(GENERATOR) author template --generator-name rust --library reqwest --output /tmp/upstream-templates >/dev/null 2>&1
	# One entry per template a pass anchors on, so none of them can stop
	# matching unnoticed: model.mustache for fix_enums and the struct pass,
	# api.mustache for fix_params_default and the error-enum pass, and
	# configuration.mustache for fix_user_agent. model.mustache is at the top
	# level; the other two live under the library subdirectory.
	#
	# `set -e`, and a flag rather than `exit 1` inside the loop. make runs the
	# recipe under plain `sh -c`, where a loop's status is its last iteration's
	# and a subshell's exit does not leave the loop — so `cmd || (echo; exit 1)`
	# printed the whole alarm and still returned 0 for every template but the
	# last one.
	@set -e; \
	drifted=""; \
	for t in model reqwest/api reqwest/configuration; do \
		if ! test -f /tmp/upstream-templates/$$t.mustache; then \
			echo "$$t.mustache is gone from the generator: scripts/fix_generated.py may no longer apply"; \
			drifted="yes"; \
		elif ! diff -u "templates/pristine/$$(basename $$t).mustache" "/tmp/upstream-templates/$$t.mustache"; then \
			echo ""; \
			echo "The generator's $$t.mustache changed. Read the diff, re-check that"; \
			echo "scripts/fix_generated.py still applies, then copy the new file over"; \
			echo "templates/pristine/."; \
			drifted="yes"; \
		fi; \
	done; \
	test -z "$$drifted"

# The two gates that stop an unattended release, both runnable by hand. The
# stuck-release issue names them as likely causes, so they need a command
# beside the name.
oasdiff: $(OASDIFF) ## Diff the live schema against the committed one, as the release does
	@$(MAKE) --no-print-directory fetch OUT=/tmp/openapi.json.new
	$(OASDIFF) breaking openapi.json /tmp/openapi.json.new \
		--severity-levels oasdiff-severity.txt --fail-on ERR

$(OASDIFF):
	curl -sfSL "https://github.com/oasdiff/oasdiff/releases/download/v$(OASDIFF_VERSION)/oasdiff_$(OASDIFF_VERSION)_$(OASDIFF_PLATFORM).tar.gz" \
		| tar -xzO oasdiff > $@
	chmod +x $@

# Needs a published baseline, so it says nothing before the first release.
semver-checks: ## Compare the Rust API against the last published crate
	cargo install cargo-semver-checks --version $(SEMVER_CHECKS_VERSION) --locked
	cargo semver-checks check-release

clean: ## Remove build output and generated docs
	rm -rf target docs .openapi-generator
