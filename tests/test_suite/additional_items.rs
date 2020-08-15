//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/additionalItems.json

mod _1_items_is_schema_no_additional_items {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/additional_items_1.json");

    #[test]
    fn r#all_items_match_schema() {
        let data = r#"[1,2,3,4,5]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _3_additional_items_as_false_without_items {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/additional_items_3.json");

    #[test]
    fn r#items_defaults_to_empty_schema_so_everything_is_valid() {
        let data = r#"[1,2,3,4,5]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_non_arrays() {
        let data = r#"{"foo":"bar"}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _4_additional_items_are_allowed_by_default {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/additional_items_4.json");

    #[test]
    fn r#only_the_first_item_is_validated() {
        let data = r#"[1,"foo",false]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
