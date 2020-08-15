//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/anyOf.json

mod _3_any_of_with_one_empty_schema {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/any_of_3.json");

    #[test]
    fn r#string_is_valid() {
        let data = r#""foo""#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#number_is_valid() {
        let data = r#"123"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
