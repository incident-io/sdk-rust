#!/usr/bin/env python3
"""Rewrite the generated client so additive API changes stay additive.

Our compatibility policy calls adding a response field, an enum value or an
optional parameter backwards-compatible, and the hourly release publishes those
without a human. In Rust each one is a hard break unless the types are shaped
for it, so this reshapes them:

1. Enums get an `Unknown(String)` catch-all. Without it a value the server
   started sending fails the *whole* response, on every already-installed copy
   of the SDK, and the release that knows the value is one those callers have
   not installed. The variant holds the string rather than being a unit, so
   reading a resource and writing it back does not replace the field with the
   literal "Unknown". That costs `Copy`, which the generator derives, and
   `Ord`/`PartialOrd` go with it — see ENUM_DERIVE_OWNED. Removing a trait
   impl from a public type later is itself breaking, so this is day-one or
   never.

2. Every struct gets `#[non_exhaustive]`, so adding a field is not a break.
   Enums get it too, so matching one requires a wildcard arm and a new value
   is not a break.

   Applied to request types as much as response types, which is the part that
   costs something. Measured against sdk-go's 124 committed schema diffs, 29 of
   them add an optional request property or an optional request parameter —
   about one release in four. Without the attribute each of those adds a field
   to a struct a caller can build with a literal, `cargo-semver-checks` rates
   it `constructible_struct_adds_field`, and the release halts before
   committing the schema, so every later run halts the same way. Roughly ten
   halts a month, each needing a human to cut a major version.

   The price is that a caller can no longer write a struct literal or
   `..Default::default()` for these types, which is why point 3 exists. It is
   what the two Rust clients in the same position do — generated from a
   spec, released continuously, stable major version. `aws-sdk-s3` marks 87%
   of its structs, including all 115 `*Input` types; `google-cloud-storage`
   marks 94% of its public model layer, including every `*Request`. Hand-
   written clients vary: `octocrab` marks about half, `azure_storage_blob`
   1.1.0 about a tenth, and `async-stripe`, `async-openai` and `cloudflare`
   none at all — all three of those are 0.x, where SemVer lets a minor break.

   Adding `#[non_exhaustive]` later is itself breaking, so this is day-one or
   never.

3. Every struct gets a constructor and a chainable setter per field, because
   `#[non_exhaustive]` leaves no other way to build one from another crate.

   `new()` takes the required fields positionally, so a missing path parameter
   stays a compile error rather than a request to `/v2/pay_reports//download`.
   The setters take the value unwrapped and return `Self`, so an optional
   field reads `.set_page_size(25)` rather than `page_size: Some(25)`.

   The generator already writes a matching `new()` for every model, so only
   the params structs need one. Both get setters.

   On a model a caller never builds, that `new()` is rewritten to take no
   arguments at all. Its required-argument form is the same problem one level
   down: the schema adding a required property widens the signature, which is
   `method_parameter_count_changed` and halts the release just as an added
   field used to. Measured over sdk-go's 125 committed schemas, that happened
   13 times in 83 days — about one a week — and every one was a
   response type. Request types keep the checked form, because the API cannot
   add a required *request* property without breaking its own wire contract,
   and that is where naming the required fields actually earns something.

   `google-cloud-storage` generates a zero-argument `new()` for all 57 of its
   request structs for the same reason; `aws-sdk-s3` keeps every input field
   `Option<T>` behind a fallible `build()`, which is the same trade made
   differently.

4. Endpoints the generator renders without a params struct get an empty
   one, so that every endpoint takes `(&Configuration, SomethingParams)`.

   The generator omits the struct when an operation has no parameters, which
   is true of 19 of 304 today. The first optional parameter added to one of
   those changes the *function's* arity rather than a struct's fields, and no
   attribute can absorb that — it is `function_parameter_count_changed`,
   Major, and it halts the release exactly as the struct cases used to.
   Adding the argument now is breaking, so this is day-one or never.

5. The per-operation `*Error` enums get `#[non_exhaustive]`. Adding a
   documented status code to an endpoint is additive on the wire and adds a
   variant here, which breaks anyone matching exhaustively. They already carry
   the generator's own `UnknownValue(serde_json::Value)` catch-all, so this is
   only about the compile-time match, not deserialization.

6. Object query parameters are flattened into the bracket form the API
   parses. The generator renders one as a single JSON-encoded value —
   `created_at={"gte":["2024-05-01"]}` — where the server reads
   `created_at[gte]=2024-05-01`, so every filter on every list endpoint is
   silently ignored.

   Two levels, not one. Most filters are `HashMap<String, Vec<String>>`, but
   `custom_field` and `incident_role` are
   `HashMap<String, HashMap<String, Vec<String>>>` and want
   `custom_field[<id>][one_of]=<value>`. A one-level flatten stringifies the
   inner map and looks fine until someone uses a custom field filter, so the
   helper recurses and both depths are tested.

   Multiple values repeat the key (`created_at[gte]=a&created_at[gte]=b`)
   rather than indexing it. reqwest percent-encodes the brackets, which is
   fine: Go's `net/url.ParseQuery` decodes them before the handler sees the
   key, so the encoded and literal forms are the same key.

   Done by patching the generator's own `parse_deep_object` rather than
   adding a second flattener beside it. It already recurses and the generator
   already routes `style: deepObject` parameters to it — `query` on
   POST /v2/alert_events/http/{id} declares that style today and calls it — so
   a second helper would mean two wire encodings in one crate, and a new `pub`
   symbol that could not be dropped without a major once the spec is fixed.
   Its array branch is the only part that is wrong.

7. The typed error is chosen by HTTP status rather than by serde. Every
   variant of a `*Error` enum holds the same `models::ErrorResponse` and the
   enum is `#[serde(untagged)]`, so deserializing picks the first variant that
   fits — always the lowest status code. A 401 arrives as `Status400`, and
   matching on `Status401` or `Status429` never fires anywhere.

8. `Configuration` and `ApiKey` get a hand-written `Debug` that redacts the
   credential. The derived one prints `bearer_access_token` in full, so
   `tracing::debug!(?config)` or a panic message leaks the API key into logs.

9. The default User-Agent names the code generator, so incident.io cannot tell
   a Rust SDK caller from any other generated client, or see which version
   they are on. sdk-go sets `incident-io-sdk-go/<version>` for the same reason.

It also asserts things the generator gets right today, so they fail loudly if
a future version stops: that every deprecated operation in the schema carries
`#[deprecated]`, and that nothing references a crate we do not depend on.

A post-generation pass rather than a forked template, because a fork fails
silently. openapi-generator opts out of mustache's fail-on-missing-key, so when
upstream renames a variable the fork still renders and quietly drops the
branch — they shipped exactly that in their own templates for 15 months. Worse,
if upstream renames or splits the file, the override is never consulted at all.
Anchoring on generated text means an upstream change shows up as this script
finding nothing, which stops the release. See the template-drift make target.

Report anything fixed here upstream, and delete it when it lands.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

# The generator derives these on every enum. Copy has to go: a variant holding
# a String cannot be Copy.
ENUM_DERIVE = (
    "#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]"
)
# Ord and PartialOrd are dropped along with Copy. Derived ordering follows
# variant declaration order, which follows schema order, so a value the API
# inserts anywhere but the end silently renumbers every later variant —
# changing how a consumer's BTreeMap or sort behaves, with cargo-semver-checks
# rating it a warning and the release shipping it as a minor. Removing a trait
# impl later is itself breaking, so this is day-one or never, like Copy.
ENUM_DERIVE_OWNED = (
    "#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]"
)

# Marks an enum this pass has already reshaped. `make generate` always starts
# from fresh generator output, so this only matters when someone runs the
# script by hand over an already-processed tree.
ALREADY_PATCHED = "#[serde(untagged)]\n    Unknown(String),"

# Floors for the passes whose true count moves with the schema. Set from the
# counts at the time of writing, rounded down hard: they exist to catch an
# anchor that stopped matching, not to pin the API's shape.
ENUM_FLOOR = 200
STRUCT_FLOOR = 1000
ERROR_ENUM_FLOOR = 200
SETTER_FLOOR = 3000
CONSTRUCTOR_FLOOR = 200
DEEP_OBJECT_FLOOR = 25
ERROR_SITE_FLOOR = 250

CATCH_ALL = """    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
"""


def enum_variant_collisions(root: Path) -> list[str]:
    """Enums that already have a variant named `Unknown`.

    Adding an enum value is backwards-compatible under our own policy and
    ships unattended, so the API is free to add one spelled "unknown" — and
    the catch-all this script appends is called `Unknown`. The result is
    "error[E0428]: the name Unknown is defined multiple times", with nothing
    in it pointing at this script. Named here instead.
    """
    offenders = []
    for file in sorted(root.glob("models/*.rs")):
        source = file.read_text()
        for match in re.finditer(r"^pub enum (\w+) \{\n(.*?)^\}", source, re.S | re.M):
            body = match.group(2)
            # The generator's own variant, not the one we append below it.
            if re.search(r"^\s+Unknown,\s*$", body, re.M) or re.search(
                r'^\s+#\[serde\(rename = "unknown"\)\]', body, re.M
            ):
                offenders.append(f"{match.group(1)} in {file}")
    return offenders


def fix_enums(root: Path) -> tuple[int, int]:
    """Add the catch-all variant, drop Copy, and mark enums non-exhaustive.

    Returns (patched, declared). Failing only when *nothing* matched would let
    a generator change that reshapes a subset through silently: the enums it
    still recognises get patched, and the rest ship with no catch-all, which is
    the exact defect this pass exists to prevent.
    """
    fixed = declared = 0
    for file in sorted(root.glob("models/*.rs")):
        source = file.read_text()
        if "pub enum " not in source:
            continue
        declared += len(re.findall(r"^pub enum ", source, re.M))
        # Already patched, from a hand-run of this script over a tree `make
        # generate` has already processed. Count it, so a second run is a
        # no-op rather than 314 alarming "unpatched" lines.
        fixed += source.count(ALREADY_PATCHED)

        # Only the derive line immediately above a `pub enum` — a file can hold
        # both a struct and its inline property enums.
        pattern = re.compile(
            r"(" + re.escape(ENUM_DERIVE) + r")\n(pub enum (\w+) \{\n)(.*?)(^\})",
            re.S | re.M,
        )

        def replace(match: re.Match[str]) -> str:
            _, header, name, body, close = match.groups()
            return (
                f"{ENUM_DERIVE_OWNED}\n#[non_exhaustive]\n{header}{body}{CATCH_ALL}{close}"
            )

        patched, count = pattern.subn(replace, source)
        if count:
            file.write_text(patched)
            fixed += count

    return fixed, declared


# The generator's derive line for a per-operation error enum. Matched exactly,
# so a change upstream shows up as this pass finding nothing.
ERROR_ENUM_HEADER = '#[derive(Debug, Clone, Serialize, Deserialize)]\n#[serde(untagged)]\npub enum '


def fix_error_enums_non_exhaustive(root: Path) -> tuple[int, int]:
    """Mark the per-operation `*Error` enums `#[non_exhaustive]`.

    One is generated per endpoint, with a variant per documented status code.
    Documenting another status is additive on the wire, and our release ships
    it without a human — but it adds a variant, so an exhaustive match in a
    consumer stops compiling and cargo-semver-checks stops the release.

    Adding the attribute is itself breaking, so this is day-one or never.

    Returns (marked, declared), so a generator change that reshapes only some
    of them fails rather than leaving the rest bare.
    """
    marked = "#[non_exhaustive]\n" + ERROR_ENUM_HEADER
    fixed = declared = 0

    for file in sorted(root.glob("apis/*.rs")):
        source = file.read_text()
        declared += len(re.findall(r"^pub enum \w+Error \{", source, re.M))
        count = source.count(ERROR_ENUM_HEADER)
        if not count:
            continue

        # Strip then add, so a hand-run over an already-processed tree is a
        # no-op rather than stacking a second attribute.
        out = source.replace(marked, ERROR_ENUM_HEADER).replace(ERROR_ENUM_HEADER, marked)
        if out != source:
            file.write_text(out)
        fixed += count

    return fixed, declared


def strip_attributes(body: str) -> str:
    """Remove doc comments and attributes from a struct body.

    Bracket-matched rather than line-based. A model field carries
    `#[serde(rename = "...")]`, and `cargo fmt` wraps a long one across several
    lines:

        #[serde(
            rename = "external_issue_reference",
            skip_serializing_if = "Option::is_none"
        )]

    A line-based strip removes the first line and leaves the rest, which then
    flattens into the previous field's type and produces a setter that does not
    parse. `make generate` runs this pass before cargo fmt, so the generator's
    one-line attributes hide it — but the script is meant to be runnable by
    hand over a formatted tree.
    """
    out = []
    index = 0
    while index < len(body):
        rest = body[index:]

        if rest.startswith("//"):
            end = body.find("\n", index)
            index = len(body) if end == -1 else end + 1
            continue

        if rest.startswith("#["):
            depth = 0
            scan = index + 1
            while scan < len(body):
                char = body[scan]
                if char == '"':
                    # Skip the string, so a bracket inside it does not count.
                    scan += 1
                    while scan < len(body) and body[scan] != '"':
                        scan += 2 if body[scan] == "\\" else 1
                elif char == "[":
                    depth += 1
                elif char == "]":
                    depth -= 1
                    if depth == 0:
                        break
                scan += 1
            index = scan + 1
            continue

        out.append(body[index])
        index += 1

    return "".join(out)


def struct_fields(body: str) -> list[tuple[str, str]]:
    """(name, type) for each field of a struct body.

    Flattens first, because `cargo fmt` wraps a long type onto its own line:

        pub incident_role:
            Option<std::collections::HashMap<String, ...>>,

    A line-by-line parse reads that field's type as the empty string, which
    then tests as not-optional. `make generate` runs this pass before cargo
    fmt so the generator's own one-line output hides it, but running the
    script by hand over a formatted tree — which every other pass supports —
    would silently misclassify the field.
    """
    flat = re.sub(r"\s+", " ", strip_attributes(body)).strip()

    # `r#` because a property named after a Rust keyword renders as a raw
    # identifier: `pub r#type: String`.
    fields = []
    for part in re.split(r"(?=\bpub (?:r#)?\w+\s*:)", flat):
        match = re.match(r"pub ((?:r#)?\w+)\s*:\s*(.+?),?\s*$", part)
        if match:
            fields.append((match.group(1), match.group(2).rstrip(",").strip()))

    return fields


def _snake(name: str) -> str:
    return re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", name).lower()


def _unwrap(ty: str, outer: str) -> tuple[str, bool]:
    """Strip one `Outer<...>` layer, if the whole type is that layer.

    Bracket-counted rather than a regex, so `Option<HashMap<String, Vec<T>>>`
    gives back the HashMap and not a truncated prefix.
    """
    prefix = outer + "<"
    if not ty.startswith(prefix) or not ty.endswith(">"):
        return ty, False

    depth = 0
    for index, char in enumerate(ty):
        if char == "<":
            depth += 1
        elif char == ">":
            depth -= 1
            if depth == 0:
                # The layer has to close at the very end, or this is something
                # like `Option<T>, U` and the prefix match was a coincidence.
                return (ty[len(prefix) : -1], True) if index == len(ty) - 1 else (ty, False)
    return ty, False


def setter(name: str, ty: str) -> str:
    """A chainable setter for one field.

    Takes the value unwrapped — `Option<Box<T>>` is set from a `T` — because
    the wrapping is an artifact of how the schema is rendered, not something
    a caller should have to reproduce at every call site.
    """
    inner, optional = _unwrap(ty, "Option")
    inner, boxed = _unwrap(inner, "Box")

    expression = "value"
    # `impl Into<String>` for String and nothing else, which is the rule
    # aws-sdk-s3 applies across 1537 of its setters: it turns
    # `.set_name("x".to_owned())` into `.set_name("x")`, and `&str` is the only
    # conversion anyone actually reaches for. Generic on other types buys
    # nothing — nothing but `bool` implements `Into<bool>` — and costs error
    # quality. Measured: no difference to build or doc time.
    #
    # Day-one: adding the generic later makes an existing `.set_x("s".into())`
    # ambiguous, and cargo-semver-checks does not catch a parameter type
    # change, so it would ship as a minor.
    if inner == "String":
        inner, expression = "impl Into<String>", f"{expression}.into()"
    if boxed:
        expression = f"Box::new({expression})"
    if optional:
        expression = f"Some({expression})"

    # `set_r#type` is not an identifier, so the method drops the raw prefix
    # the field has to carry.
    method = name.removeprefix("r#")

    return (
        f"    /// Sets `{name}`.\n"
        # Consuming setter: `params.set_page_size(100);` as a statement
        # compiles, drops the result and does nothing, with no warning.
        # Verified against an external crate before adding this.
        f"    #[must_use]\n"
        f"    pub fn set_{method}(mut self, value: {inner}) -> Self {{\n"
        f"        self.{name} = {expression};\n"
        f"        self\n"
        f"    }}\n"
    )


def _pascal(snake: str) -> str:
    """The generator's own snake-to-Pascal for an operation name.

    Verified against all 285 operations that already have a params struct:
    this reproduces every one of their names exactly, which is what makes it
    safe to invent the missing 19.
    """
    return "".join(part[:1].upper() + part[1:] for part in snake.split("_"))


def fix_missing_params(root: Path) -> tuple[int, int]:
    """Give every endpoint a params struct, even the ones with no parameters.

    See point 4 in the module docstring. Runs before fix_non_exhaustive and
    fix_constructors, which then mark and equip these like any other.

    Returns (added, still bare). The caller asserts on the second: "how many
    did this run add" falls to zero on the second pass of `make generate`, and
    "how many endpoints have no parameters" falls as the API grows, so neither
    is something to put a floor under.
    """
    # Whitespace-tolerant: this pass runs before cargo fmt, on the generator's
    # own one-line signature, but the script is also runnable by hand over a
    # formatted tree where rustfmt has split it across lines. Matching only
    # one form made the pass a no-op on the first run and a 19-file rewrite on
    # the second.
    signature = re.compile(
        r"pub async fn (\w+)\(\s*configuration: &configuration::Configuration,?\s*\)(\s*->)"
    )

    added = 0
    for file in sorted(root.glob("apis/*_api.rs")):
        source = file.read_text()
        matches = list(signature.finditer(source))
        if not matches:
            continue

        # Two phases. Rewriting the signatures right-to-left keeps each
        # match's offsets valid, but the struct declarations go at the top of
        # the file and would shift every offset below them — so they are all
        # collected and inserted once, afterwards.
        out = source
        declarations = []

        for match in reversed(matches):
            operation = match.group(1)
            name = _pascal(operation) + "Params"

            out = (
                out[: match.start()]
                + f"pub async fn {operation}(\n"
                + f"    configuration: &configuration::Configuration,\n"
                + f"    params: {name},\n"
                + f"){match.group(2)}"
                + out[match.end() :]
            )

            # A use for it, so an empty struct does not warn. It goes away on
            # its own the day the API adds a parameter and the generator
            # starts reading the struct.
            body = out.index(" {\n", match.start()) + len(" {\n")
            out = out[:body] + "    let _ = params;\n" + out[body:]

            declarations.append(
                f"/// struct for passing parameters to the method [`{operation}`]\n"
                f"///\n"
                f"/// This operation takes no parameters today. The struct exists so that\n"
                f"/// the first one the API adds is a field rather than a change to this\n"
                f"/// function's signature, which no attribute can absorb.\n"
                f"#[derive(Clone, Debug)]\n"
                f"pub struct {name} {{}}\n\n"
            )
            added += 1

        anchor = out.index("/// struct for typed errors of method")
        out = out[:anchor] + "".join(reversed(declarations)) + out[anchor:]
        file.write_text(out)

    bare = sum(
        len(signature.findall(file.read_text()))
        for file in sorted(root.glob("apis/*_api.rs"))
    )
    return added, bare


def _reachable(root: Path, roots: set[str]) -> set[str]:
    """Every model reachable from `roots`, following `models::X` references.

    One traversal for both directions. The two callers differ only in their
    seed set, and `fix_response_model_constructors` subtracts one result from
    the other — so a fix applied to one copy and not the other would move
    models silently between "keeps a checked constructor" and "does not",
    which no assertion here would catch.
    """
    seen: set[str] = set()
    queue = list(roots)
    while queue:
        name = queue.pop()
        if name in seen:
            continue
        seen.add(name)
        nested = root / "models" / f"{_snake(name)}.rs"
        if nested.exists():
            queue += re.findall(r"models::(\w+)", nested.read_text())

    return seen


def request_reachable(root: Path) -> set[str]:
    """Models a caller sends: anything a `*Params` field can reach."""
    roots: set[str] = set()
    for file in sorted(root.glob("apis/*_api.rs")):
        for match in re.finditer(
            r"pub struct \w+Params \{\n(.*?)^\}", file.read_text(), re.S | re.M
        ):
            roots |= set(re.findall(r"models::(\w+)", match.group(1)))

    return _reachable(root, roots)


def response_reachable(root: Path) -> set[str]:
    """Models a caller receives: every endpoint's return type, transitively.

    The API adds required properties to these — measured at 13 times in 83
    days over sdk-go's history — and a required-argument constructor turns
    each of those into `method_parameter_count_changed`, which halts the
    release.
    """
    roots: set[str] = set()
    for file in sorted(root.glob("apis/*_api.rs")):
        # Whitespace-tolerant: cargo fmt wraps a long signature, and the
        # strict form misses 36 of 253 return types on a formatted tree. Those
        # models then read as request-only and keep a required-argument
        # constructor — the halt this split exists to prevent. `make generate`
        # runs the pass on unformatted output first, so the shipped tree was
        # right, but running the script by hand was not.
        roots |= set(
            re.findall(r"->\s*Result<\s*models::(\w+)\s*,\s*Error<", file.read_text())
        )

    return _reachable(root, roots)


def fix_response_model_constructors(root: Path) -> tuple[int, int]:
    """Rewrite `new(required...)` to `new()` on models a caller never builds.

    Returns (rewritten, left checked). See point 3 in the module docstring for
    why the split, and why it falls where it does.

    Every model derives `Default` — the generator emits a `Default` impl for
    the enums too — so `Default::default()` is a complete value.
    """
    # Request-reachable *and not* returned anywhere. A model in both sets is
    # one the API can add a required property to as a backwards-compatible
    # response change, which would widen a checked constructor and halt the
    # release. 38 models are in both, so the checked set is narrower than
    # "everything a caller can build".
    reachable = request_reachable(root) - response_reachable(root)
    rewritten = checked = 0

    for file in sorted(root.glob("models/*.rs")):
        source = file.read_text()

        # `-> Self` as well as `-> Name`, so a second run recognises the form
        # this pass already wrote and reports the same count. Without it the
        # numbers collapse on re-run and the floor below fires on a tree that
        # is already correct.
        # `[^\n]` for the doc-comment lines and `[^)]` for the argument list,
        # not `.` — this runs under re.S, where `.` crosses newlines, and a
        # repeated group of those backtracks catastrophically. The first
        # version of this regex ran for minutes at 100% CPU on the real tree
        # instead of the second it takes now.
        match = re.search(
            r"impl (\w+) \{\n(?:    ///[^\n]*\n)*    pub fn new\(([^)]*)\)"
            r" -> (?:\1|Self) \{\n.*?\n    \}\n\}",
            source,
            re.S,
        )
        if not match:
            continue

        name, args = match.group(1), match.group(2).strip()
        if name in reachable:
            checked += 1
            continue
        if not args:
            # Already argument-free, from the generator or an earlier run.
            rewritten += 1
            continue

        source = source[: match.start()] + (
            f"impl {name} {{\n"
            f"    /// A value with every field at its default.\n"
            f"    ///\n"
            f"    /// This is a response type, so you receive one rather than\n"
            f"    /// building it. Set the fields you need with the `set_*`\n"
            f"    /// methods below — deliberately not a required-argument\n"
            f"    /// constructor, because then the schema adding a required\n"
            f"    /// property would change this signature and break you.\n"
            f"    pub fn new() -> Self {{\n"
            f"        Default::default()\n"
            f"    }}\n"
            f"}}"
        ) + source[match.end() :]
        file.write_text(source)
        rewritten += 1

    return rewritten, checked


# The four types the generator writes from its own templates rather than from
# the schema. Named rather than globbed, because `apis/mod.rs` and
# `apis/configuration.rs` hold nothing else we want to touch, and because the
# constructor each one needs is different.
TEMPLATE_TYPES = {
    "apis/configuration.rs": ("Configuration", "ApiKey"),
    "apis/mod.rs": ("ResponseContent", "Error"),
}


def fix_template_types(root: Path) -> int:
    """Mark the generator's own `Configuration`, `ApiKey`, `ResponseContent`
    and `Error`, and give the two that lack one a constructor.

    An API change cannot add a field to these, which is why they were exempt.
    But the generator can: `configuration.mustache` renders `aws_v4_key` and
    `token_source` behind flags, and `api_mod.mustache` has a conditional
    `ReqwestMiddleware` variant on `Error`. A generator bump would then force
    a major version on a crate built to avoid them.

    Removing the attribute later is not breaking, so this is the reversible
    direction. `ResponseContent` gets a constructor because mocking an error
    response needs one, and `ApiKey` because it has no other way in.
    """
    marked = 0

    for relative, names in TEMPLATE_TYPES.items():
        file = root / relative
        source = file.read_text()
        out = source

        for name in names:
            for keyword in ("struct", "enum"):
                for suffix in (" {", "<T> {"):
                    header = f"pub {keyword} {name}{suffix}"
                    if header not in out:
                        continue
                    out = out.replace(f"#[non_exhaustive]\n{header}", header, 1).replace(
                        header, f"#[non_exhaustive]\n{header}", 1
                    )
                    marked += 1

        # Constructors, for the two with none. Anchored on their absence so a
        # future generator that grows one does not get a duplicate.
        if "impl<T> ResponseContent<T> {" not in out and "pub struct ResponseContent<T>" in out:
            out += (
                "\nimpl<T> ResponseContent<T> {\n"
                "    /// The parts of a non-2xx response.\n"
                "    ///\n"
                "    /// Present so that a consumer can build one in a test. The type\n"
                "    /// is `#[non_exhaustive]`, so a struct literal will not do.\n"
                "    pub fn new(status: reqwest::StatusCode, content: String, entity: Option<T>) -> Self {\n"
                "        Self {\n"
                "            status,\n"
                "            content,\n"
                "            entity,\n"
                "        }\n"
                "    }\n"
                "}\n"
            )

        if "impl ApiKey {" not in out and "pub struct ApiKey {" in out:
            out += (
                "\nimpl ApiKey {\n"
                "    /// Unused by this API, which authenticates with a bearer token.\n"
                "    /// Present because the type is `#[non_exhaustive]` and would\n"
                "    /// otherwise be unconstructible.\n"
                "    pub fn new(key: String) -> Self {\n"
                "        Self { prefix: None, key }\n"
                "    }\n"
                "}\n"
            )

        if out != source:
            file.write_text(out)

    return marked


def fix_model_constructor_args(root: Path) -> int:
    """Widen `String` arguments on the generator's model `new()` to
    `impl Into<String>`.

    Without this the setters take a `&str` and the constructor beside them
    still demands a `String`, which is the more jarring half: the required
    fields are exactly the ones a caller cannot avoid passing.
    """
    widened = 0

    for file in sorted(root.glob("models/*.rs")):
        source = file.read_text()
        match = re.search(
            r"(    pub fn new\()([^)]*?)(\) -> (?:\w+|Self) \{\n)(.*?)(^    \}$)",
            source,
            re.S | re.M,
        )
        if not match or "String" not in match.group(2):
            continue

        names = [
            argument.split(":", 1)[0].strip()
            for argument in match.group(2).split(",")
            if argument.strip().endswith(": String")
        ]
        if not names:
            continue

        args = ", ".join(
            f"{a.split(':', 1)[0].strip()}: impl Into<String>"
            if a.strip().endswith(": String")
            else a.strip()
            for a in match.group(2).split(",")
            if a.strip()
        )

        body = match.group(4)
        for name in names:
            # The initialiser is shorthand (`name,`) for every required field.
            body = re.sub(rf"^(\s+){re.escape(name)},$", rf"\1{name}: {name}.into(),", body, flags=re.M)

        file.write_text(
            source[: match.start()]
            + match.group(1) + args + match.group(3) + body + match.group(5)
            + source[match.end() :]
        )
        widened += 1

    return widened


def fix_non_exhaustive(root: Path) -> tuple[int, int]:
    """Mark every generated struct `#[non_exhaustive]`.

    Every schema-derived one, not a chosen subset: a struct a caller only ever
    receives and a struct a caller builds are both broken by an added field,
    and the API adds fields to both. Point 2 in the module docstring has the
    measurement.

    `apis/configuration.rs` and `apis/mod.rs` are skipped here and handled by
    fix_template_types instead, which marks them too but has to write their
    constructors by hand.

    Returns (marked, declared). A floor on `marked` alone would only catch the
    denominator collapsing; the caller compares the two, so a generator change
    that reshapes *some* structs cannot leave the rest unmarked and ship.
    """
    marked = declared = 0

    for file in sorted(list(root.glob("models/*.rs")) + list(root.glob("apis/*_api.rs"))):
        source = file.read_text()
        out = source

        for match in re.finditer(r"^pub struct (\w+)[ <{]", source, re.M):
            name = match.group(1)
            declared += 1
            # Anchored on the declaration, and strip-then-add, so running this
            # over an already-processed tree is a no-op rather than stacking a
            # second attribute, which rustc rejects.
            for suffix in (" {", ";", "("):
                header = f"pub struct {name}{suffix}"
                if header in out:
                    out = out.replace(f"#[non_exhaustive]\n{header}", header, 1).replace(
                        header, f"#[non_exhaustive]\n{header}", 1
                    )
                    marked += 1
                    break

        if out != source:
            file.write_text(out)

    return marked, declared


def fix_constructors(root: Path) -> tuple[int, int, int]:
    """Give every struct a way to be built, now that no literal can be.

    Anything without a constructor gets `new(required...)`, which in practice
    means the params structs. Models already have a matching `new()` from the
    generator, so they only get the setters.

    Same file set as fix_non_exhaustive, since it anchors on the attribute
    that pass writes.

    Returns (constructors written, setters, structs left with neither).

    That third number is the one that matters. A `#[non_exhaustive]` struct
    with no constructor and no setters cannot be built from another crate at
    all, and nothing else notices: it compiles, clippy is happy, the tests
    pass, and cargo-semver-checks has no baseline for a struct the same schema
    change introduced. It is zero today and the caller fails the build if it
    ever isn't.
    """
    constructors = setters = unequipped = 0

    for file in sorted(list(root.glob("models/*.rs")) + list(root.glob("apis/*_api.rs"))):
        source = file.read_text()
        # Everything this pass writes goes below this marker, so a re-run
        # regenerates it wholesale rather than appending a second copy.
        marker = "\n// --- generated by scripts/fix_generated.py ---\n"
        base = source.split(marker)[0].rstrip("\n")

        blocks: list[str] = []
        defaults: list[str] = []
        # `{}` as well as a multi-line body: an endpoint with no parameters
        # gets an empty params struct, and cargo fmt writes that as `{}`.
        for match in re.finditer(
            r"^#\[non_exhaustive\]\npub struct (\w+) (?:\{\}|\{\n(.*?)^\})", base, re.S | re.M
        ):
            name, body = match.group(1), match.group(2) or ""
            fields = struct_fields(body)
            if not fields and not name.endswith("Params"):
                # No fields on a model is legitimate — an empty `type: object`
                # schema — and the generator still writes it a new(). Only a
                # struct with neither fields nor a constructor is unbuildable.
                if f"impl {name} {{" not in base:
                    unequipped += 1
                continue

            block = [f"impl {name} {{"]

            # Only where the generator wrote none. It writes one for every
            # model, and a second would not compile.
            if not re.search(rf"^impl {re.escape(name)} \{{\n(?:.*?\n)*?    pub fn new\(", base, re.M):
                required = [(n, t) for n, t in fields if not t.startswith("Option<")]
                # Same String rule as the setters, so a path parameter reads
                # `Foo::new("01ABC")` rather than `Foo::new("01ABC".to_owned())`.
                args = ", ".join(
                    f"{n}: impl Into<String>" if t == "String" else f"{n}: {t}"
                    for n, t in required
                )
                inits = "\n".join(
                    (
                        f"            {n}: {n}.into(),"
                        if t == "String"
                        else f"            {n},"
                    )
                    if not t.startswith("Option<")
                    else f"            {n}: None,"
                    for n, t in fields
                )
                initialiser = f"Self {{\n{inits}\n        }}" if fields else "Self {}"
                block.append(
                    f"    /// The required parameters. Set the optional ones with the\n"
                    f"    /// `set_*` methods below.\n"
                    f"    #[must_use]\n"
                    f"    pub fn new({args}) -> Self {{\n"
                    f"        {initialiser}\n"
                    f"    }}\n"
                )
                constructors += 1

                # clippy::new_without_default, and it is right: a no-argument
                # `new()` should have a `Default`. Only where there are no
                # required fields — a `Default` for a struct with a required
                # path parameter would hand back an empty string and request
                # `/v2/pay_reports//download`.
                if not required:
                    defaults.append(
                        f"impl Default for {name} {{\n"
                        f"    fn default() -> Self {{\n"
                        f"        Self::new()\n"
                        f"    }}\n"
                        f"}}"
                    )

            for field_name, field_type in fields:
                block.append(setter(field_name, field_type))
                setters += 1

            block.append("}")
            blocks.append("\n".join(block))

        # Guarded, like every other pass. This one rewrote all 1,196 files on
        # every run even when the content was identical, which was most of the
        # I/O in the second pass of `make generate`.
        if blocks:
            written = "\n\n".join(blocks + defaults)
            out = base + "\n" + marker + "\n" + written + "\n"
        else:
            out = base + "\n"
        if out != source:
            file.write_text(out)

    return constructors, setters, unequipped


# The generator's rendering of an object query parameter, before and after
# cargo fmt has had a chance to wrap it. Anchored on the exact text so an
# upstream change shows up as this pass matching nothing.
# Two shapes, because the generator picks a branch from the parameter's
# declared style and the schema has had both:
#
#   no style          -> one JSON-encoded value: created_at={"gte":[...]}
#   deepObject+explode -> a loop that drops the name: gte=["..."]
#
# Neither is what the API parses. Both are matched so the pass survives the
# spec changing under it in either direction.
DEEP_OBJECT_CALLS = (
    # The form emitted when the parameter declares no style.
    re.compile(
        r"req_builder\s*=\s*req_builder\s*\.query\(&\[\(\s*\"(\w+)\",\s*"
        r"&serde_json::to_string\(param_value\)\?\s*\)\]\);"
    ),
    # The form emitted for style: deepObject with explode: true.
    re.compile(
        r"if let Some\(ref param_value\) = params\.(\w+)\s*\{\s*"
        r"let mut query_params = Vec::with_capacity\(param_value\.len\(\)\);\s*"
        r"for \(key, value\) in param_value\.iter\(\)\s*\{\s*"
        r"query_params\.push\(\(key\.to_string\(\),\s*serde_json::to_string\(value\)\?\)\);\s*"
        r"\}\s*"
        r"req_builder = req_builder\.query\(&query_params\);\s*"
        r"\}"
    ),
)

# The tell for a JSON-encoded query value, whatever shape surrounds it. Used
# for the residue check instead of the anchors above: counting unrewritten
# instances of a known form reports zero when the generator switches to a form
# the pass does not know, which is exactly when the check needs to fire.
JSON_ENCODED = "serde_json::to_string("

# The format string only the generator's indexed version contains, used to
# tell "not patched yet" from "already patched" in a way cargo fmt cannot
# disturb.
INDEXED_FORMAT = '"{}[{}][{}]"'

# parse_deep_object's array branch, which indexes where the API repeats.
INDEXED_ARRAY = """                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                }"""

REPEATED_ARRAY = """                // Repeated, not indexed: the API reads
                // `created_at[gte]=a&created_at[gte]=b`, and an indexed key
                // makes its decoder assign rather than append, so all but the
                // last value is silently dropped.
                //
                // Handled inline rather than by recursing. The generator's
                // version recurses on every element, and this function accepts
                // only an object at the top — so an array of strings, which is
                // the shape of every filter this API has, falls through to the
                // `unimplemented!` below and panics at runtime. Only an object
                // element recurses now.
                serde_json::Value::Array(array) => {
                    for item in array.iter() {
                        match item {
                            serde_json::Value::Object(_) => params.append(
                                &mut parse_deep_object(&format!("{}[{}]", prefix, key), item),
                            ),
                            serde_json::Value::String(s) => {
                                params.push((format!("{}[{}]", prefix, key), s.clone()))
                            }
                            _ => params.push((format!("{}[{}]", prefix, key), item.to_string())),
                        }
                    }
                }"""


def fix_deep_object_params(root: Path) -> tuple[int, int]:
    """Send object query parameters the way the API parses them.

    Two edits. The generator's `parse_deep_object` indexes array values, so
    its array branch is rewritten to repeat the key. Then every object
    parameter the generator rendered as one JSON blob is pointed at it.

    Returns (call sites using the helper, call sites still JSON-encoded). The
    second is the one that matters and must be zero: a floor would let a
    half-matching anchor through, and a filter that ships JSON-encoded is
    invisible — the request succeeds and returns unfiltered results.
    """
    module = root / "apis" / "mod.rs"
    source = module.read_text()
    if INDEXED_ARRAY in source:
        module.write_text(source.replace(INDEXED_ARRAY, REPEATED_ARRAY, 1))
    elif INDEXED_FORMAT in source:
        # The indexed format string is still there but the block around it is
        # not what we matched, so the generator reshaped the helper and the
        # rewrite below would point call sites at something unknown.
        raise SystemExit(
            f"parse_deep_object still indexes array values ({INDEXED_FORMAT}) but its "
            f"array branch is not the text this pass matches — check "
            f"templates/pristine/api_mod.mustache against the generator."
        )
    # Otherwise the branch is already repeated. Detected by the absence of the
    # indexed format string rather than by matching our own replacement text:
    # cargo fmt rewrites what we emit, so on the second run of `make generate`
    # an exact-text check sees neither form and wrongly reports drift.

    flattened = remaining = 0
    for file in sorted(root.glob("apis/*_api.rs")):
        source = out = file.read_text()

        # The first form replaces the `.query(...)` call in place; the second
        # matches the whole `if let` block, so its replacement rebuilds it.
        out = DEEP_OBJECT_CALLS[0].sub(
            lambda m: (
                "req_builder = req_builder.query(&crate::apis::parse_deep_object("
                f'"{m.group(1)}", &serde_json::to_value(param_value)?));'
            ),
            out,
        )
        out = DEEP_OBJECT_CALLS[1].sub(
            lambda m: (
                f"if let Some(ref param_value) = params.{m.group(1)} {{\n"
                "        req_builder = req_builder.query(&crate::apis::parse_deep_object("
                f'"{m.group(1)}", &serde_json::to_value(param_value)?));\n'
                "    }"
            ),
            out,
        )

        if out != source:
            file.write_text(out)

        # Counted from `out`, inside this loop: a second pass over the same
        # files just to tally them reads all 59 again for nothing.
        flattened += out.count("crate::apis::parse_deep_object(")
        remaining += out.count(JSON_ENCODED)

    return flattened, remaining


ERROR_ENTITY = re.compile(
    r"let entity: Option<(\w+Error)> = serde_json::from_str\(&content\)\.ok\(\);"
)


def fix_error_status_selection(root: Path) -> tuple[int, int]:
    """Choose the error variant from the HTTP status, not from serde.

    The generated code deserializes the body into an `#[serde(untagged)]` enum
    whose variants all hold `models::ErrorResponse`, so serde returns the
    first one that fits and every error arrives as the lowest status code the
    endpoint documents. Each enum gets a `from_status` that maps the real
    status onto the right variant, and the call sites use it.

    Returns (call sites rewritten, enums given a from_status).
    """
    rewritten = mapped = 0

    for file in sorted(root.glob("apis/*_api.rs")):
        source = file.read_text()
        out = ERROR_ENTITY.sub(
            lambda m: (
                f"let entity: Option<{m.group(1)}> = "
                f"serde_json::from_str::<models::ErrorResponse>(&content)\n"
                f"            .ok()\n"
                f"            .map(|body| {m.group(1)}::from_status(status.as_u16(), body));"
            ),
            source,
        )
        # From the rewritten text, not from matches of the pre-rewrite anchor:
        # `make generate` runs the pass twice and the second run legitimately
        # matches nothing, so counting the anchor reports zero on a correct
        # tree. Third time I have made this mistake in this file.
        rewritten += out.count("::from_status(status.as_u16(), body)")

        # One `from_status` per enum, built from the variants it declares.
        impls = []
        for match in re.finditer(r"^pub enum (\w+Error) \{\n(.*?)^\}", out, re.S | re.M):
            name, body = match.group(1), match.group(2)
            if f"impl {name} {{" in out:
                continue
            statuses = re.findall(r"^\s*Status(\d+)\(", body, re.M)
            if not statuses:
                continue
            arms = "\n".join(
                f"            {code} => Self::Status{code}(body)," for code in statuses
            )
            impls.append(
                f"impl {name} {{\n"
                f"    /// The variant matching the response's HTTP status.\n"
                f"    ///\n"
                f"    /// Not `serde`: every variant holds the same type and the enum\n"
                f"    /// is `#[serde(untagged)]`, so deserializing would always return\n"
                f"    /// the lowest status code the endpoint documents.\n"
                f"    fn from_status(status: u16, body: models::ErrorResponse) -> Self {{\n"
                f"        match status {{\n{arms}\n"
                f"            _ => Self::UnknownValue(\n"
                f"                serde_json::to_value(body).unwrap_or(serde_json::Value::Null),\n"
                f"            ),\n"
                f"        }}\n"
                f"    }}\n"
                f"}}"
            )
            mapped += 1

        if impls:
            out = out.rstrip("\n") + "\n\n" + "\n\n".join(impls) + "\n"

        if out != source:
            file.write_text(out)

    return rewritten, mapped


REDACTED_DEBUG = '''
// Hand-written, because the derived one prints the credential. A
// `tracing::debug!(?config)` or a panic message would otherwise put the API
// key in logs, which is the kind of thing that ends up in a support bundle.
impl std::fmt::Debug for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Configuration")
            .field("base_path", &self.base_path)
            .field("user_agent", &self.user_agent)
            .field("client", &self.client)
            .field("basic_auth", &self.basic_auth.as_ref().map(|_| "***"))
            .field("oauth_access_token", &self.oauth_access_token.as_ref().map(|_| "***"))
            .field("bearer_access_token", &self.bearer_access_token.as_ref().map(|_| "***"))
            .field("api_key", &self.api_key.as_ref().map(|_| "***"))
            .finish()
    }
}

impl std::fmt::Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ApiKey")
            .field("prefix", &self.prefix)
            .field("key", &"***")
            .finish()
    }
}
'''


def fix_debug_redaction(root: Path) -> int:
    """Stop `{:?}` printing the API key.

    `Configuration` derives `Debug` and holds `bearer_access_token`; `ApiKey`
    holds `key`. Both are replaced with an impl that prints `***`.
    """
    file = root / "apis" / "configuration.rs"
    source = file.read_text()
    out = source

    redacted = 0
    for name in ("Configuration", "ApiKey"):
        derive = f"#[derive(Debug, Clone)]\n#[non_exhaustive]\npub struct {name} {{"
        if derive in out:
            out = out.replace(
                derive, f"#[derive(Clone)]\n#[non_exhaustive]\npub struct {name} {{", 1
            )
        if f"impl std::fmt::Debug for {name}" in out:
            redacted += 1

    if "impl std::fmt::Debug for Configuration" not in out:
        out = out.rstrip("\n") + "\n" + REDACTED_DEBUG
        redacted = 2

    if out != source:
        file.write_text(out)

    return redacted


def fix_user_agent(root: Path) -> int:
    """Name this SDK and its version in the User-Agent."""
    file = root / "apis" / "configuration.rs"
    source = file.read_text()
    old = 'user_agent: Some("OpenAPI-Generator/1.0.0/rust".to_owned()),'
    if old not in source:
        # Already patched by a hand-run over a processed tree, like the passes
        # above. Only a missing marker means the generator changed it.
        return 1 if "incident-io-sdk-rust/" in source else 0

    file.write_text(
        source.replace(
            old,
            'user_agent: Some(concat!("incident-io-sdk-rust/", env!("CARGO_PKG_VERSION")).to_owned()),',
        )
    )
    return 1


def count_deprecated(root: Path, spec: Path) -> tuple[int, int]:
    """Compare `#[deprecated]` attributes against the schema's deprecated
    operations.

    The generator emits these itself, which is why there is no fix for it here
    — but nothing else would notice if a future version stopped, and the
    attribute is the only signal a caller gets.
    """
    marked = sum(
        len(re.findall(r"^#\[deprecated", file.read_text(), re.M))
        for file in sorted(root.glob("apis/*.rs"))
    )

    document = json.loads(spec.read_text())
    expected = sum(
        1
        for path in document["paths"].values()
        for method, operation in path.items()
        if method in ("get", "post", "put", "patch", "delete")
        and isinstance(operation, dict)
        and operation.get("deprecated")
    )

    return marked, expected


def assert_no_undeclared_crates(root: Path) -> dict[str, str]:
    """Crates the generator declares for schema shapes we do not have.

    serde_repr appears for an integer-backed enum, serde_with for a nullable
    optional property or a `format: byte` field. Our Cargo.toml lists neither,
    so the day the schema grows one of those shapes the generated code
    references a crate we do not depend on. Named here rather than left to
    rustc, whose error gives no hint about what to do.
    """
    crates = ("serde_repr", "serde_with")
    offenders: dict[str, str] = {}

    for file in sorted(root.rglob("*.rs")):
        source = file.read_text()
        for crate in crates:
            if crate not in offenders and crate in source:
                offenders[crate] = str(file)
        if len(offenders) == len(crates):
            break

    return offenders


def main(src: str, spec: str) -> int:
    root = Path(src)
    if not root.is_dir():
        sys.exit(f"{root} is not a directory")

    missing_params, bare_endpoints = fix_missing_params(root)
    template_types = fix_template_types(root)
    collisions = enum_variant_collisions(root)
    enums, declared_enums = fix_enums(root)
    structs, declared_structs = fix_non_exhaustive(root)
    error_enums, declared_error_enums = fix_error_enums_non_exhaustive(root)
    # Before fix_constructors, which appends below a marker this one does not
    # know about, and after fix_non_exhaustive for no reason but readability.
    response_models, checked_models = fix_response_model_constructors(root)
    widened = fix_model_constructor_args(root)
    constructors, setters, unequipped = fix_constructors(root)
    deep_objects, json_encoded = fix_deep_object_params(root)
    error_sites, error_enums_mapped = fix_error_status_selection(root)
    redacted = fix_debug_redaction(root)
    agent = fix_user_agent(root)
    marked, expected_deprecations = count_deprecated(root, Path(spec))
    undeclared = assert_no_undeclared_crates(root)

    print(f"Fixed: {enums} of {declared_enums} enums (catch-all, non_exhaustive, Copy/Ord dropped)")
    print(f"Fixed: {structs} of {declared_structs} structs (non_exhaustive)")
    print(
        f"Fixed: {error_enums} of {declared_error_enums} per-operation error "
        f"enums (non_exhaustive)"
    )
    print(f"Wrote: {constructors} params constructors and {setters} setters")
    print(
        f"Relaxed: {response_models} response models to new(); "
        f"{checked_models} request models keep their required arguments"
    )
    print(f"Added: {missing_params} params structs for parameterless endpoints")
    # From the pass, not recounted: a proxy over the tree counted any model
    # with a String *setter* and reported 667 where the real figure is 140.
    # The number differs between the two runs of `make generate` because the
    # second legitimately widens nothing, which is why it is only printed.
    print(f"Widened: {widened} model constructors to impl Into<String> (first run only)")
    print(f"Marked: {template_types} generator-template types (non_exhaustive)")
    print(f"Flattened: {deep_objects} object query parameters to the bracket form")
    print(
        f"Fixed: {error_sites} error sites now select by HTTP status "
        f"({error_enums_mapped} enums)"
    )
    print(f"Fixed: {redacted} of 2 credential-holding types redact in Debug")
    print(f"Fixed: user agent ({agent} file)")
    print(f"Checked: {marked} of {expected_deprecations} deprecated operations marked")

    failures = []
    for offender in collisions:
        failures.append(
            f"{offender} already has an `Unknown` variant, which collides with the "
            f"catch-all this script appends. The schema has grown an enum value "
            f"spelled \"unknown\". Rename CATCH_ALL's variant — note that is a "
            f"breaking change to every consumer matching on it."
        )
    if enums != declared_enums:
        failures.append(
            f"{declared_enums - enums} enum(s) left unpatched — the generator emits "
            f"a shape this does not recognise, and those enums will reject any "
            f"value added to them"
        )
    # Both counts above come from the same files, so if the generator stopped
    # emitting `pub enum` altogether the comparison is 0 != 0 and passes, with
    # no enum carrying a catch-all. The other passes carry a floor for exactly
    # this; this one needs one too.
    if declared_enums < ENUM_FLOOR:
        failures.append(
            f"only {declared_enums} enums found, expected at least {ENUM_FLOOR} — "
            f"the generator stopped rendering schema enums as Rust enums, so the "
            f"catch-all pass has nothing to attach to"
        )
    # Equality, not just a floor. A floor catches the generator emitting none
    # of a shape; only the declared count catches it emitting a *variant* this
    # pass does not recognise, which leaves the remainder bare and shipping.
    # fix_enums has done this from the start; these two did not.
    if error_enums != declared_error_enums:
        failures.append(
            f"{declared_error_enums - error_enums} error enum(s) left unmarked — "
            f"the generator renders some of them a way this does not recognise, "
            f"and documenting another status code on those endpoints will break "
            f"every caller matching on them"
        )
    if error_enums < ERROR_ENUM_FLOOR:
        failures.append(
            f"only {error_enums} error enums found, expected at least "
            f"{ERROR_ENUM_FLOOR} — the generator stopped emitting them"
        )
    # Floors, not exact counts: every number here moves whenever the API adds
    # an endpoint or a model. Zero means an anchor stopped matching entirely; a
    # collapse means it stopped matching most of them, which is the same defect
    # in a form a "not zero" check would wave through.
    if structs != declared_structs:
        failures.append(
            f"{declared_structs - structs} struct(s) left unmarked — the generator "
            f"renders some of them a way this does not recognise, and adding a "
            f"property to those will halt the release"
        )
    if structs < STRUCT_FLOOR:
        failures.append(
            f"only {structs} structs found, expected at least {STRUCT_FLOOR} — "
            f"the generator stopped emitting them"
        )
    if unequipped:
        failures.append(
            f"{unequipped} struct(s) are #[non_exhaustive] with no constructor and "
            f"no setters, so no other crate can build them at all. Either the "
            f"field parse stopped matching for them, or the generator stopped "
            f"writing a new() it used to write."
        )
    if setters < SETTER_FLOOR:
        failures.append(
            f"only {setters} setters were written, expected at least "
            f"{SETTER_FLOOR} — the field parse stopped matching, so those structs "
            f"are now #[non_exhaustive] with no way to build them"
        )
    if template_types != 4:
        failures.append(
            f"{template_types} of 4 generator-template types were marked "
            f"non_exhaustive — the generator renamed or reshaped Configuration, "
            f"ApiKey, ResponseContent or Error, so a later generator bump that "
            f"adds a field to them would force a major version"
        )
    if constructors < CONSTRUCTOR_FLOOR:
        failures.append(
            f"only {constructors} params constructors were written, expected at "
            f"least {CONSTRUCTOR_FLOOR} — the generator changed how it renders a "
            f"params struct, or useSingleRequestParameter stopped taking effect. "
            f"Those endpoints are now uncallable."
        )
    # Equality, not a floor: every model is either request-reachable or it is
    # not, so the two counts have to add up to the number of model files. A
    # floor would let most of a partial failure through, and the models it let
    # through are exactly the ones that then halt the release when the API adds
    # a required property to them.
    model_files = len(list(root.glob("models/*.rs"))) - 1  # less mod.rs
    if response_models + checked_models != model_files:
        failures.append(
            f"{response_models} relaxed + {checked_models} left checked = "
            f"{response_models + checked_models}, but there are {model_files} "
            f"models — the new() anchor stopped matching for the difference, so "
            f"those keep a required-argument constructor and will halt the "
            f"release when the API adds a required property to them"
        )
    # Zero remaining, and measured independently of the rewrite anchors. An
    # earlier version counted unrewritten instances of the one form it knew, so
    # when the schema gained `style: deepObject` and the generator switched
    # forms entirely, the count was zero and this passed while every filter
    # shipped JSON-encoded. That failure is invisible downstream: the request
    # succeeds and returns unfiltered results.
    if error_sites < ERROR_SITE_FLOOR:
        failures.append(
            f"only {error_sites} error sites select the variant by HTTP status, "
            f"expected at least {ERROR_SITE_FLOOR} — the generator changed how it "
            f"builds the error entity. Those endpoints fall back to serde on an "
            f"untagged enum, which always returns the lowest status code, so "
            f"matching on 401 or 429 silently never fires."
        )
    if redacted != 2:
        failures.append(
            f"{redacted} of 2 credential types redact in Debug — the generator "
            f"reshaped Configuration or ApiKey, so `{{:?}}` would print the API key."
        )
    if json_encoded:
        failures.append(
            f"{json_encoded} query value(s) are still JSON-encoded in apis/*_api.rs "
            f"— the generator emits an object parameter in a shape this pass does "
            f"not rewrite. Those filters are silently ignored by the API: the "
            f"request succeeds and returns unfiltered results. Compare a generated "
            f"call site against DEEP_OBJECT_CALLS."
        )
    # Zero remaining, not a floor on how many were added. The count of
    # parameterless endpoints is *designed* to fall as the API grows — each
    # empty struct gains fields and stops being empty — so a floor on it would
    # fail a correct tree, blaming the anchor for something the API did.
    if bare_endpoints:
        failures.append(
            f"{bare_endpoints} endpoint(s) still take only a &Configuration — the "
            f"signature anchor matched some and not others. The first parameter "
            f"the API adds to one of those changes its arity, which no attribute "
            f"can absorb and no later release can undo."
        )
    if not agent:
        failures.append(
            "the User-Agent default did not match — the generator changed it, so "
            "this SDK's traffic is unattributable"
        )
    if marked != expected_deprecations:
        failures.append(
            f"{expected_deprecations} operations are deprecated in the schema but "
            f"{marked} carry #[deprecated] — the generator stopped emitting them, "
            f"and callers get no warning"
        )
    for crate, first in sorted(undeclared.items()):
        failures.append(
            f"{crate} is referenced by the generated code but Cargo.toml does not "
            f"depend on it: the schema has grown a shape that needs it. Add the "
            f"dependency, or change the schema. First seen in {first}"
        )

    if failures:
        print()
        for failure in failures:
            print(f"  {failure}", file=sys.stderr)
        print(
            "\nThe generated client is not safe to publish as-is. Check whether "
            "openapi-generator changed its output, or fixed this upstream — in "
            "which case delete the fix here.",
            file=sys.stderr,
        )
        return 1

    return 0


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} <src-dir> <openapi.json>", file=sys.stderr)
        raise SystemExit(2)
    raise SystemExit(main(sys.argv[1], sys.argv[2]))
