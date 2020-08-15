//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/pattern.json

mod _1_pattern_is_not_anchored {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/pattern_1.json");

    #[test]
    fn r#matches_a_substring() {
        let data = r#""xxaayy""#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
