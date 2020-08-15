//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/items.json

mod _3_nested_items {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/items_3.json");

    #[test]
    fn r#valid_nested_array() {
        let data = r#"[[[[1]],[[2],[3]]],[[[4],[5],[6]]]]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#nested_array_with_invalid_type() {
        let data = r#"[[[["1"]],[[2],[3]]],[[[4],[5],[6]]]]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#not_deep_enough() {
        let data = r#"[[[1],[2],[3]],[[4],[5],[6]]]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}
