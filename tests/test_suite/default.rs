//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/default.json

mod _0_invalid_type_for_default {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/default_0.json");

    #[test]
    fn r#valid_when_property_is_specified() {
        let data = r#"{"foo":13}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#still_valid_when_the_invalid_default_is_used() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _1_invalid_string_value_for_default {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/default_1.json");

    #[test]
    fn r#valid_when_property_is_specified() {
        let data = r#"{"bar":"good"}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#still_valid_when_the_invalid_default_is_used() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
