use serde_json;

use serde_derive::{Deserialize, Serialize};

schemafy::schemafy!(
    "tests/elastic-apm/spec/root.json"
);

#[test]
fn elastic_apm_schema() {
    let os1: Option<Span> = None;
}
