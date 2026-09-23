//! The README's filtering examples.
//!
//! Filtering is the most-used feature of the list endpoints and the one whose
//! wire format `scripts/fix_generated.py` patches, so the examples belong
//! under test like the rest. The wire format itself is covered by
//! `tests/client.rs`; this file checks the shapes the README shows compile and
//! land in the right fields.

use incident_io::apis::incidents_v2_api::IncidentsV2ListParams;
use std::collections::HashMap;

#[test]
fn a_one_level_filter_reaches_its_field() {
    let mut created_at = HashMap::new();
    created_at.insert("gte".to_owned(), vec!["2024-05-01".to_owned()]);

    let mut status = HashMap::new();
    status.insert(
        "one_of".to_owned(),
        vec!["01ABC".to_owned(), "01DEF".to_owned()],
    );

    let params = IncidentsV2ListParams::new()
        .set_created_at(created_at)
        .set_status(status);

    assert_eq!(params.created_at.unwrap()["gte"], vec!["2024-05-01"]);
    assert_eq!(params.status.unwrap()["one_of"].len(), 2);
}

#[test]
fn a_custom_field_filter_nests_one_level_further() {
    let mut operators = HashMap::new();
    operators.insert("one_of".to_owned(), vec!["01VALUE".to_owned()]);

    let mut custom_field = HashMap::new();
    custom_field.insert("01FIELD".to_owned(), operators);

    let params = IncidentsV2ListParams::new().set_custom_field(custom_field);

    assert_eq!(
        params.custom_field.unwrap()["01FIELD"]["one_of"],
        vec!["01VALUE"]
    );
}

/// Every setter consumes `self` and returns `Self`, so dropping the result is
/// a silent no-op. `#[must_use]` turns that into a warning — this test is here
/// to say the attribute is deliberate, since nothing else would fail if the
/// pass stopped emitting it.
#[test]
fn setters_are_must_use() {
    let params = IncidentsV2ListParams::new().set_page_size(100);

    assert_eq!(params.page_size, Some(100));
}
