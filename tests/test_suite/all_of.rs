//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/allOf.json

mod _0_all_of {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/all_of_0.json");

    #[test]
    fn r#all_of() {
        let data = r#"{"bar":2,"foo":"baz"}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#mismatch_second() {
        let data = r#"{"foo":"baz"}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#mismatch_first() {
        let data = r#"{"bar":2}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#wrong_type() {
        let data = r#"{"bar":"quux","foo":"baz"}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _3_all_of_with_one_empty_schema {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/all_of_3.json");

    #[test]
    fn r#any_data_is_valid() {
        let data = r#"1"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _4_all_of_with_two_empty_schemas {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/all_of_4.json");

    #[test]
    fn r#any_data_is_valid() {
        let data = r#"1"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
