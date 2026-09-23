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
   costs something. Measured against sdk-go's 124 committed schemas, 30 of
   them add an optional request property or an optional request parameter —
   about one release in four. Without the attribute each of those adds a field
   to a struct a caller can build with a literal, `cargo-semver-checks` rates
   it `constructible_struct_adds_field`, and the release halts before
   committing the schema, so every later run halts the same way. Roughly ten
   halts a month, each needing a human to cut a major version.

   The price is that a caller can no longer write a struct literal or
   `..Default::default()` for these types, which is why point 3 exists. It is
   what the two Rust SDKs in the same position do: `aws-sdk-s3` marks 86% of
   its structs including every `*Input`, and `google-cloud-storage` marks
   every request and response in its public layer. Everything that still takes
   a bare struct literal — async-stripe, async-openai, octocrab, the Azure and
   Cloudflare clients — is 0.x, where SemVer lets a minor break and the
   question never arises.

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

4. The per-operation `*Error` enums get `#[non_exhaustive]`. Adding a
   documented status code to an endpoint is additive on the wire and adds a
   variant here, which breaks anyone matching exhaustively. They already carry
   the generator's own `UnknownValue(serde_json::Value)` catch-all, so this is
   only about the compile-time match, not deserialization.

5. The default User-Agent names the code generator, so incident.io cannot tell
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

CATCH_ALL = """    /// A value this build of the SDK does not know about.
    ///
    /// The API adds enum values as a backwards-compatible change. This holds
    /// the value verbatim and serializes back to it unchanged, so writing back
    /// a resource you read does not discard it.
    #[serde(untagged)]
    Unknown(String),
"""


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


def fix_error_enums_non_exhaustive(root: Path) -> int:
    """Mark the per-operation `*Error` enums `#[non_exhaustive]`.

    One is generated per endpoint, with a variant per documented status code.
    Documenting another status is additive on the wire, and our release ships
    it without a human — but it adds a variant, so an exhaustive match in a
    consumer stops compiling and cargo-semver-checks stops the release.

    Adding the attribute is itself breaking, so this is day-one or never.
    """
    marked = "#[non_exhaustive]\n" + ERROR_ENUM_HEADER
    fixed = 0

    for file in sorted(root.glob("apis/*.rs")):
        source = file.read_text()
        count = source.count(ERROR_ENUM_HEADER)
        if not count:
            continue

        # Strip then add, so a hand-run over an already-processed tree is a
        # no-op rather than stacking a second attribute.
        out = source.replace(marked, ERROR_ENUM_HEADER).replace(ERROR_ENUM_HEADER, marked)
        if out != source:
            file.write_text(out)
        fixed += count

    return fixed


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
    if boxed:
        expression = f"Box::new({expression})"
    if optional:
        expression = f"Some({expression})"

    # `set_r#type` is not an identifier, so the method drops the raw prefix
    # the field has to carry.
    method = name.removeprefix("r#")

    return (
        f"    /// Sets `{name}`.\n"
        f"    pub fn set_{method}(mut self, value: {inner}) -> Self {{\n"
        f"        self.{name} = {expression};\n"
        f"        self\n"
        f"    }}\n"
    )


def fix_non_exhaustive(root: Path) -> int:
    """Mark every generated struct `#[non_exhaustive]`.

    Every schema-derived one, not a chosen subset: a struct a caller only ever
    receives and a struct a caller builds are both broken by an added field,
    and the API adds fields to both. Point 2 in the module docstring has the
    measurement.

    `apis/configuration.rs` and `apis/mod.rs` are left alone on purpose. They
    hold `Configuration`, `ApiKey` and `ResponseContent`, which come from the
    generator's own templates rather than from the schema: they cannot grow a
    field because the API changed, only because a human bumped the generator.
    Marking them would cost a caller the struct literal for `Configuration`
    and the struct pattern for `ResponseContent`, and buy nothing.
    """
    marked = 0

    for file in sorted(list(root.glob("models/*.rs")) + list(root.glob("apis/*_api.rs"))):
        source = file.read_text()
        out = source

        for match in re.finditer(r"^pub struct (\w+)[ <{]", source, re.M):
            name = match.group(1)
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

    return marked


def fix_constructors(root: Path) -> tuple[int, int]:
    """Give every struct a way to be built, now that no literal can be.

    Anything without a constructor gets `new(required...)`, which in practice
    means the params structs. Models already have a matching `new()` from the
    generator, so they only get the setters.

    Same file set as fix_non_exhaustive, since it anchors on the attribute
    that pass writes.

    Returns (constructors written, setters).
    """
    constructors = setters = 0

    for file in sorted(list(root.glob("models/*.rs")) + list(root.glob("apis/*_api.rs"))):
        source = file.read_text()
        # Everything this pass writes goes below this marker, so a re-run
        # regenerates it wholesale rather than appending a second copy.
        marker = "\n// --- generated by scripts/fix_generated.py ---\n"
        base = source.split(marker)[0].rstrip("\n")

        blocks: list[str] = []
        defaults: list[str] = []
        for match in re.finditer(r"^#\[non_exhaustive\]\npub struct (\w+) \{\n(.*?)^\}", base, re.S | re.M):
            name, body = match.group(1), match.group(2)
            fields = struct_fields(body)
            if not fields:
                continue

            block = [f"impl {name} {{"]

            # Only where the generator wrote none. It writes one for every
            # model, and a second would not compile.
            if not re.search(rf"^impl {re.escape(name)} \{{\n(?:.*?\n)*?    pub fn new\(", base, re.M):
                required = [(n, t) for n, t in fields if not t.startswith("Option<")]
                args = ", ".join(f"{n}: {t}" for n, t in required)
                inits = "\n".join(
                    f"            {n}," if not t.startswith("Option<") else f"            {n}: None,"
                    for n, t in fields
                )
                block.append(
                    f"    /// The required parameters. Set the optional ones with the\n"
                    f"    /// `set_*` methods below.\n"
                    f"    pub fn new({args}) -> Self {{\n"
                    f"        Self {{\n{inits}\n        }}\n"
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

        if blocks:
            written = "\n\n".join(blocks + defaults)
            file.write_text(base + "\n" + marker + "\n" + written + "\n")
        elif source != base + "\n":
            file.write_text(base + "\n")

    return constructors, setters


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

    enums, declared_enums = fix_enums(root)
    structs = fix_non_exhaustive(root)
    error_enums = fix_error_enums_non_exhaustive(root)
    # After fix_non_exhaustive, which is what it anchors on.
    constructors, setters = fix_constructors(root)
    agent = fix_user_agent(root)
    marked, expected_deprecations = count_deprecated(root, Path(spec))
    undeclared = assert_no_undeclared_crates(root)

    print(f"Fixed: {enums} of {declared_enums} enums (catch-all, non_exhaustive, Copy/Ord dropped)")
    print(f"Fixed: {structs} structs (non_exhaustive)")
    print(f"Fixed: {error_enums} per-operation error enums (non_exhaustive)")
    print(f"Wrote: {constructors} params constructors and {setters} setters")
    print(f"Fixed: user agent ({agent} file)")
    print(f"Checked: {marked} of {expected_deprecations} deprecated operations marked")

    failures = []
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
    if error_enums < ERROR_ENUM_FLOOR:
        failures.append(
            f"only {error_enums} error enums got non_exhaustive, expected at least "
            f"{ERROR_ENUM_FLOOR} — the generator changed how it renders them, and "
            f"documenting another status code will now break the release"
        )
    # Floors, not exact counts: every number here moves whenever the API adds
    # an endpoint or a model. Zero means an anchor stopped matching entirely; a
    # collapse means it stopped matching most of them, which is the same defect
    # in a form a "not zero" check would wave through.
    if structs < STRUCT_FLOOR:
        failures.append(
            f"only {structs} structs got non_exhaustive, expected at least "
            f"{STRUCT_FLOOR} — the generator changed how it renders a struct, and "
            f"adding a property will now break the release"
        )
    if setters < SETTER_FLOOR:
        failures.append(
            f"only {setters} setters were written, expected at least "
            f"{SETTER_FLOOR} — the field parse stopped matching, so those structs "
            f"are now #[non_exhaustive] with no way to build them"
        )
    if not constructors:
        failures.append(
            "no params constructors were written — the generator changed how it "
            "renders a params struct, or useSingleRequestParameter stopped taking "
            "effect. Every endpoint is now uncallable."
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
