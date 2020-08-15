//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/type.json

mod _0_integer_type_matches_integers {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_0.json");

    #[test]
    fn r#an_integer_is_an_integer() {
        let data = r#"1"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#a_float_is_not_an_integer() {
        let data = r#"1.1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_not_an_integer() {
        let data = r#""foo""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_still_not_an_integer_even_if_it_looks_like_one() {
        let data = r#""1""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_object_is_not_an_integer() {
        let data = r#"{}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_array_is_not_an_integer() {
        let data = r#"[]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_boolean_is_not_an_integer() {
        let data = r#"true"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#null_is_not_an_integer() {
        let data = r#"null"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _1_number_type_matches_numbers {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_1.json");

    #[test]
    fn r#an_integer_is_a_number() {
        let data = r#"1"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#a_float_is_a_number() {
        let data = r#"1.1"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#a_string_is_not_a_number() {
        let data = r#""foo""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_still_not_a_number_even_if_it_looks_like_one() {
        let data = r#""1""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_object_is_not_a_number() {
        let data = r#"{}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_array_is_not_a_number() {
        let data = r#"[]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_boolean_is_not_a_number() {
        let data = r#"true"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#null_is_not_a_number() {
        let data = r#"null"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _2_string_type_matches_strings {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_2.json");

    #[test]
    fn r#_1_is_not_a_string() {
        let data = r#"1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_float_is_not_a_string() {
        let data = r#"1.1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_a_string() {
        let data = r#""foo""#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#a_string_is_still_a_string_even_if_it_looks_like_a_number() {
        let data = r#""1""#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#an_empty_string_is_still_a_string() {
        let data = r#""""#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#an_object_is_not_a_string() {
        let data = r#"{}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_array_is_not_a_string() {
        let data = r#"[]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_boolean_is_not_a_string() {
        let data = r#"true"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#null_is_not_a_string() {
        let data = r#"null"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _3_object_type_matches_objects {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_3.json");

    #[test]
    fn r#an_integer_is_not_an_object() {
        let data = r#"1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_float_is_not_an_object() {
        let data = r#"1.1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_not_an_object() {
        let data = r#""foo""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_object_is_an_object() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#an_array_is_not_an_object() {
        let data = r#"[]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_boolean_is_not_an_object() {
        let data = r#"true"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#null_is_not_an_object() {
        let data = r#"null"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _4_array_type_matches_arrays {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_4.json");

    #[test]
    fn r#an_integer_is_not_an_array() {
        let data = r#"1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_float_is_not_an_array() {
        let data = r#"1.1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_not_an_array() {
        let data = r#""foo""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_object_is_not_an_array() {
        let data = r#"{}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_array_is_an_array() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#a_boolean_is_not_an_array() {
        let data = r#"true"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#null_is_not_an_array() {
        let data = r#"null"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _5_boolean_type_matches_booleans {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_5.json");

    #[test]
    fn r#an_integer_is_not_a_boolean() {
        let data = r#"1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#zero_is_not_a_boolean() {
        let data = r#"0"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_float_is_not_a_boolean() {
        let data = r#"1.1"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#a_string_is_not_a_boolean() {
        let data = r#""foo""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_empty_string_is_not_a_boolean() {
        let data = r#""""#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_object_is_not_a_boolean() {
        let data = r#"{}"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#an_array_is_not_a_boolean() {
        let data = r#"[]"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }

    #[test]
    fn r#true_is_a_boolean() {
        let data = r#"true"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#false_is_a_boolean() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#null_is_not_a_boolean() {
        let data = r#"null"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}

mod _8_type_as_array_with_one_item {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/type_8.json");

    #[test]
    fn r#string_is_valid() {
        let data = r#""foo""#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#number_is_invalid() {
        let data = r#"123"#;
        assert!(serde_json::from_str::<Schema>(&data).is_err());
    }
}
