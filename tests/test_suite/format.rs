//! Automatically generated from tests/JSON-Schema-Test-Suite/tests/draft4/format.json

mod _0_validation_of_e_mail_addresses {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/format_0.json");

    #[test]
    fn r#ignores_integers() {
        let data = r#"12"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_floats() {
        let data = r#"13.7"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_objects() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_arrays() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_booleans() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_null() {
        let data = r#"null"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _1_validation_of_ip_addresses {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/format_1.json");

    #[test]
    fn r#ignores_integers() {
        let data = r#"12"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_floats() {
        let data = r#"13.7"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_objects() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_arrays() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_booleans() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_null() {
        let data = r#"null"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _2_validation_of_i_pv_6_addresses {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/format_2.json");

    #[test]
    fn r#ignores_integers() {
        let data = r#"12"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_floats() {
        let data = r#"13.7"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_objects() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_arrays() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_booleans() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_null() {
        let data = r#"null"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _3_validation_of_hostnames {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/format_3.json");

    #[test]
    fn r#ignores_integers() {
        let data = r#"12"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_floats() {
        let data = r#"13.7"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_objects() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_arrays() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_booleans() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_null() {
        let data = r#"null"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _4_validation_of_date_time_strings {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/format_4.json");

    #[test]
    fn r#ignores_integers() {
        let data = r#"12"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_floats() {
        let data = r#"13.7"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_objects() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_arrays() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_booleans() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_null() {
        let data = r#"null"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}

mod _5_validation_of_ur_is {
    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: Schema "tests/test_suite/schemas/format_5.json");

    #[test]
    fn r#ignores_integers() {
        let data = r#"12"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_floats() {
        let data = r#"13.7"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_objects() {
        let data = r#"{}"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_arrays() {
        let data = r#"[]"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_booleans() {
        let data = r#"false"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }

    #[test]
    fn r#ignores_null() {
        let data = r#"null"#;
        let _: Schema = serde_json::from_str(&data).unwrap();
    }
}
