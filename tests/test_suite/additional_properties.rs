//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/additionalProperties.json

mod _4_additional_properties_are_allowed_by_default {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/additional_properties_4.json");

    #[test]
    fn r#additional_properties_are_allowed() {
        let data = r#"{"bar":2,"foo":1,"quux":true}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
