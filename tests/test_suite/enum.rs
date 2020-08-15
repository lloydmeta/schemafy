//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/enum.json

mod _2_enums_in_properties {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/enum_2.json");

    #[test]
    fn r#both_properties_are_valid() {
        let data = r#"{"bar":"bar","foo":"foo"}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#missing_optional_property_is_valid() {
        let data = r#"{"bar":"bar"}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#missing_required_property_is_invalid() {
        let data = r#"{"foo":"foo"}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#missing_all_properties_is_invalid() {
        let data = r#"{}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}
