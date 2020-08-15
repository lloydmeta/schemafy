//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/required.json

mod _1_required_default_validation {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/required_1.json");

    #[test]
    fn r#not_required_by_default() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
