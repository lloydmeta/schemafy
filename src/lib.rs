// This would be nice once it stabilizes:
// https://github.com/rust-lang/rust/issues/44732
// #![feature(external_doc)]
// #![doc(include = "../README.md")]

//! This is a Rust crate which can take a [json schema (draft
//! 4)](http://json-schema.org/) and generate Rust types which are
//! serializable with [serde](https://serde.rs/). No checking such as
//! `min_value` are done but instead only the structure of the schema
//! is followed as closely as possible.
//!
//! As a schema could be arbitrarily complex this crate makes no
//! guarantee that it can generate good types or even any types at all
//! for a given schema but the crate does manage to bootstrap itself
//! which is kind of cool.
//!
//! ## Example
//!
//! Generated types for VS Codes [debug server protocol][]: <https://docs.rs/debugserver-types>
//!
//! [debug server protocol]:https://code.visualstudio.com/docs/extensions/example-debuggers
//!
//! ## Usage
//!
//! Rust types can be generated by passing a path to a JSON schema to the [`schemafy`]
//! procedural macro.
//!
//! ```rust
//! extern crate serde;
//! extern crate schemafy_core;
//! extern crate serde_json;
//!
//! use serde::{Serialize, Deserialize};
//!
//! schemafy::schemafy!(
//!     "tests/nested.json"
//! );
//!
//! schemafy::schemafy!(
//!     root: Schema // Optional name for the root type (if one exists)
//!     "schemafy_lib/src/schema.json"
//! );
//!
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let nested: Defnested = serde_json::from_str(r#"{ "append": "abc" }"#)?;
//!     assert_eq!(nested.append, Some("abc".to_string()));
//!     Ok(())
//! }
//! ```

use std::path::PathBuf;

use schemafy_lib::{Expander, Schema};
use std::rc::Rc;

/// A configurable builder for generating Rust types from a JSON
/// schema.
///
/// The default options are usually fine. In that case, you can use
/// the [`generate()`](fn.generate.html) convenience method instead.
struct GenerateBuilder<'a> {
    /// The name of the root type defined by the schema. If the schema
    /// does not define a root type (some schemas are simply a
    /// collection of definitions) then simply pass `None`.
    pub root_name: Option<String>,
    /// The module path to this crate. Some generated code may make
    /// use of types defined in this crate. Unless you have
    /// re-exported this crate or imported it under a different name,
    /// the default should be fine.
    pub schemafy_path: &'a str,

    /// Optional function for manipulating types ad-hoc
    type_replacer: Option<Box<dyn Fn(&str) -> Option<String>>>,
}

impl<'a> Default for GenerateBuilder<'a> {
    fn default() -> Self {
        GenerateBuilder {
            root_name: None,
            schemafy_path: "::schemafy_core::",
            type_replacer: None,
        }
    }
}

impl<'a> GenerateBuilder<'a> {
    fn build_tokens(mut self, tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
        struct Def {
            root: Option<String>,
            input_file: syn::LitStr,
        }

        impl syn::parse::Parse for Def {
            fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
                let root = if input.peek(syn::Ident) {
                    let root_ident: syn::Ident = input.parse()?;
                    if root_ident != "root" {
                        return Err(syn::Error::new(root_ident.span(), "Expected `root`"));
                    }
                    input.parse::<syn::Token![:]>()?;
                    Some(input.parse::<syn::Ident>()?.to_string())
                } else {
                    None
                };
                Ok(Def {
                    root,
                    input_file: input.parse()?,
                })
            }
        }

        let def = syn::parse_macro_input!(tokens as Def);
        self.root_name = def.root;

        let input_file = PathBuf::from(def.input_file.value());
        let crate_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

        let input_path = if input_file.is_relative() {
            crate_root.join(input_file)
        } else {
            input_file
        };

        let input_dir = input_path
            .parent()
            .expect(&format!(
                "Could not detect directory of file: {}",
                def.input_file.value()
            ))
            .to_owned();

        let json = std::fs::read_to_string(&input_path).unwrap_or_else(|err| {
            panic!("Unable to read `{}`: {}", input_path.to_string_lossy(), err)
        });

        let schema: Rc<Schema> =
            Rc::new(serde_json::from_str(&json).unwrap_or_else(|err| panic!("{}", err)));
        let mut expander = Expander::new(
            self.root_name.as_ref().map(|s| &**s),
            self.schemafy_path,
            Rc::clone(&schema),
            input_dir,
            &self.type_replacer,
        );
        expander.expand(schema).1.into()
    }
}

/// Generate Rust types from a JSON schema.
///
/// If the `root` parameter is supplied, then a type will be
/// generated from the root of the schema.
///
/// ```rust
/// extern crate serde;
/// extern crate schemafy_core;
/// extern crate serde_json;
///
/// use serde::{Serialize, Deserialize};
///
/// schemafy::schemafy!(
///     root: MyRoot // Optional name for the root type (if one exists)
///     "tests/nested.json"
/// );
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let nested: Defnested = serde_json::from_str(r#"{ "append": "abc" }"#)?;
///     assert_eq!(nested.append, Some("abc".to_string()));
///     Ok(())
/// }
/// ```
#[proc_macro]
pub fn schemafy(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    GenerateBuilder {
        ..GenerateBuilder::default()
    }
    .build_tokens(tokens)
}

#[doc(hidden)]
#[proc_macro]
pub fn regenerate(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::process::Command;

    let mut gen_builder = GenerateBuilder {
        ..GenerateBuilder::default()
    };
    gen_builder.type_replacer = Some(Box::new(|typ| {
        if typ == "Schema" {
            Some(format!("::std::rc::Rc<{}>", typ))
        } else {
            None
        }
    }));

    let tokens = gen_builder.build_tokens(tokens);

    {
        let out = tokens.to_string();
        std::fs::write("src/schema.rs", &out).unwrap();
        Command::new("rustfmt")
            .arg("schemafy_lib/src/schema.rs")
            .output()
            .unwrap();
    }

    tokens
}
