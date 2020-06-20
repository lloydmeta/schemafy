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
//! Rust code is generated by providing a [`Schema`](./struct.Schema.html) struct (which can be deserailized from JSON).
//!
//! A proc macro is availible in [`schemafy`](https://docs.rs/schemafy) crate
//!
//! ```rust
//! extern crate serde;
//! extern crate schemafy_core;
//! extern crate serde_json;
//!
//! use serde::{Serialize, Deserialize};
//! use schemafy_lib::Expander;
//! use std::path::PathBuf;
//!
//! let json = std::fs::read_to_string("src/schema.json").expect("Read schema JSON file");
//!
//! let schema = serde_json::from_str(&json).unwrap();
//! let mut expander = Expander::new(
//!     Some("Schema"),
//!     "::schemafy_core::",
//!     &schema,
//!     PathBuf::from("src")
//! );
//!
//! let code = expander.expand(&schema);
//! ```

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate quote;

/// Types from the JSON Schema meta-schema (draft 4).
///
/// This module is itself generated from a JSON schema.
mod schema;

use inflector::Inflector;

use serde_json::Value;

pub use schema::{Schema, SimpleTypes};

use proc_macro2::{Span, TokenStream};

use std::collections::HashMap;
use std::path::{PathBuf, Path};

fn replace_invalid_identifier_chars(s: &str) -> String {
    s.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
}

pub fn str_to_ident(s: &str) -> syn::Ident {
    let s = replace_invalid_identifier_chars(s);
    let keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
        "where", "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
        "typeof", "unsized", "virtual", "yield", "async", "await", "try",
    ];

    if keywords.iter().any(|&keyword| keyword == s) {
        syn::Ident::new(&format!("{}_", s), Span::call_site())
    } else {
        syn::Ident::new(&format!("{}", s), Span::call_site())
    }
}

fn rename_keyword(prefix: &str, s: &str) -> Option<TokenStream> {
    let n = str_to_ident(s);

    if n != s {
        let prefix = syn::Ident::new(prefix, Span::call_site());
        Some(quote! {
            #[serde(rename = #s)]
            #prefix #n
        })
    } else {
        None
    }
}

fn field(s: &str) -> TokenStream {
    if let Some(t) = rename_keyword("pub", s) {
        t
    } else {
        let snake = s.to_snake_case();
        if snake != s || snake.contains(|c: char| c == '$' || c == '#') {
            let field = if snake == "ref" {
                syn::Ident::new("ref_".into(), Span::call_site())
            } else {
                syn::Ident::new(&snake.replace('$', "").replace('#', ""), Span::call_site())
            };

            quote! {
                #[serde(rename = #s)]
                pub #field
            }
        } else {
            let field = syn::Ident::new(s, Span::call_site());
            quote!( pub #field )
        }
    }
}

fn merge_option<T, F>(mut result: &mut Option<T>, r: &Option<T>, f: F)
where
    F: FnOnce(&mut T, &T),
    T: Clone,
{
    *result = match (&mut result, r) {
        (&mut &mut Some(ref mut result), &Some(ref r)) => return f(result, r),
        (&mut &mut None, &Some(ref r)) => Some(r.clone()),
        _ => return (),
    };
}

fn merge_all_of(result: &mut Schema, r: &Schema) {
    use std::collections::btree_map::Entry;

    for (k, v) in &r.properties {
        match result.properties.entry(k.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(v.clone());
            }
            Entry::Occupied(mut entry) => merge_all_of(entry.get_mut(), v),
        }
    }

    if let Some(ref ref_) = r.ref_ {
        result.ref_ = Some(ref_.clone());
    }

    if let Some(ref description) = r.description {
        result.description = Some(description.clone());
    }

    merge_option(&mut result.required, &r.required, |required, r_required| {
        required.extend(r_required.iter().cloned());
    });

    result.type_.retain(|e| r.type_.contains(e));
}

const LINE_LENGTH: usize = 100;
const INDENT_LENGTH: usize = 4;

fn make_doc_comment(mut comment: &str, remaining_line: usize) -> TokenStream {
    let mut out_comment = String::new();
    out_comment.push_str("/// ");
    let mut length = 4;
    while let Some(word) = comment.split(char::is_whitespace).next() {
        if comment.is_empty() {
            break;
        }
        comment = &comment[word.len()..];
        if length + word.len() >= remaining_line {
            out_comment.push_str("\n/// ");
            length = 4;
        }
        out_comment.push_str(word);
        length += word.len();
        let mut n = comment.chars();
        match n.next() {
            Some('\n') => {
                out_comment.push_str("\n");
                out_comment.push_str("/// ");
                length = 4;
            }
            Some(_) => {
                out_comment.push_str(" ");
                length += 1;
            }
            None => (),
        }
        comment = n.as_str();
    }
    if out_comment.ends_with(' ') {
        out_comment.pop();
    }
    out_comment.push_str("\n");
    out_comment.parse().unwrap()
}

struct FieldExpander<'a, 'r: 'a> {
    default: bool,
    expander: &'a mut Expander<'r>,
}

impl<'a, 'r> FieldExpander<'a, 'r> {
    fn expand_fields(&mut self, type_name: &str, schema: &Schema) -> Vec<TokenStream> {
        let schema = self.expander.schema(schema);
        schema
            .properties
            .iter()
            .map(|(field_name, value)| {
                self.expander.current_field.clone_from(field_name);
                let key = field(field_name);
                let required = schema
                    .required
                    .iter()
                    .flat_map(|a| a.iter())
                    .any(|req| req == field_name);
                // if the schema of the field is a reference, then expand it and add it to the
                // current Schema's types
                if let Some(_) = value.ref_ {
                    self.expander.schema(value);
                }
                value.items.iter().for_each(|i| {
                    self.expander.schema(i);
                });
                let field_type = self.expander.expand_type(type_name, required, value);
                if !field_type.typ.starts_with("Option<") {
                    self.default = false;
                }
                let typ = field_type.typ.parse::<TokenStream>().unwrap();

                let default = if field_type.default {
                    Some(quote! { #[serde(default)] })
                } else {
                    None
                };
                let attributes = if field_type.attributes.is_empty() {
                    None
                } else {
                    let attributes = field_type
                        .attributes
                        .iter()
                        .map(|attr| attr.parse::<TokenStream>().unwrap());
                    Some(quote! {
                        #[serde( #(#attributes),* )]
                    })
                };
                let comment = value
                    .description
                    .as_ref()
                    .map(|comment| make_doc_comment(comment, LINE_LENGTH - INDENT_LENGTH));
                quote! {
                    #comment
                    #default
                    #attributes
                    #key : #typ
                }
            })
            .collect()
    }
}

pub struct Expander<'r> {
    root_name: Option<&'r str>,
    schemafy_path: &'r str,
    root: &'r Schema,
    current_type: String,
    current_field: String,
    types: Vec<(String, TokenStream)>,
    schema_directory: PathBuf,
    resolved_schemas: HashMap<PathBuf, Schema>,
}

struct FieldType {
    typ: String,
    attributes: Vec<String>,
    default: bool,
}

impl<S> From<S> for FieldType
where
    S: Into<String>,
{
    fn from(s: S) -> FieldType {
        FieldType {
            typ: s.into(),
            attributes: Vec::new(),
            default: false,
        }
    }
}

impl<'r> Expander<'r> {
    pub fn new(
        root_name: Option<&'r str>,
        schemafy_path: &'r str,
        root: &'r Schema,
        schema_directory: PathBuf,
    ) -> Expander<'r> {
        Expander {
            root_name,
            root,
            schemafy_path,
            current_field: "".into(),
            current_type: "".into(),
            types: Vec::new(),
            schema_directory,
            resolved_schemas: HashMap::new(),
        }
    }

    fn type_ref(&mut self, s: &str) -> String {
        let as_path = PathBuf::from(s);
        let s = if s == "#" {
            self.root_name.expect("No root name specified for schema")
        } else if self.is_resolved_ref_path(&as_path) {
            Self::type_from_json_file(&as_path)
        } else {
            s.split('/').last().expect("Component")
        };
        replace_invalid_identifier_chars(&s.to_pascal_case())
    }

    fn schema(&mut self, schema: &Schema) -> Schema {
        let schema = match schema.ref_ {
            Some(ref ref_) => self.schema_ref(ref_),
            None => schema.clone(),
        };
        match schema.all_of {
            Some(ref all_of) if !all_of.is_empty() => {
                all_of
                    .iter()
                    .skip(1)
                    .fold(self.schema(&all_of[0]).clone(), |mut result, def| {
                        merge_all_of(&mut result, &self.schema(def));
                        result
                    })
            }
            _ => schema.clone(),
        }
    }

    fn schema_ref(&mut self, s: &str) -> Schema {
        let (schema, ref_lookup) = if s.contains(".json") {
            // Format referenced.json#/definitions/ExternalType
            let path_split_from_rest: Vec<&str> = s.split('#').collect::<Vec<&str>>();
            let (ref_path, maybe_inner_lookup) = {
                let path_str = path_split_from_rest[0];
                let ref_file = PathBuf::from(path_str);
                let ref_path = if ref_file.is_relative() {
                    self.schema_directory.join(ref_file)
                } else {
                    ref_file
                };
                (
                    ref_path,
                    path_split_from_rest
                        .get(1)
                        .map(|s| s.trim_start_matches('/')),
                )
            };
            let resolved_schema = self.expand_file_schema_ref(&ref_path);
            if let Some(inner_ref_lookup) = maybe_inner_lookup {
                (resolved_schema, inner_ref_lookup)
            } else {
                return resolved_schema;
            }
        } else {
            (self.root.clone(), s)
        };

        ref_lookup.split('/').fold(schema, |schema, comp: &str| {
            if comp == "#" {
                self.root.clone()
            } else if comp == "definitions" {
                schema
            } else {
                schema
                    .definitions
                    .get(comp)
                    .unwrap_or_else(|| panic!("Expected definition: `{}` {}", s, comp))
                    .clone()
            }
        })
    }

    fn expand_type(&mut self, type_name: &str, required: bool, typ: &Schema) -> FieldType {
        let mut result = self.expand_type_(typ);
        if type_name == result.typ {
            result.typ = format!("Box<{}>", result.typ)
        }
        if !required && !result.default {
            result.typ = format!("Option<{}>", result.typ)
        }
        result
    }

    fn expand_type_(&mut self, typ: &Schema) -> FieldType {
        if let Some(ref ref_) = typ.ref_ {
            self.type_ref(ref_).into()
        } else if typ.any_of.as_ref().map_or(false, |a| a.len() == 2) {
            let any_of = typ.any_of.as_ref().unwrap();
            let simple = self.schema(&any_of[0]);
            let array = self.schema(&any_of[1]);
            if !array.type_.is_empty() {
                if let SimpleTypes::Array = array.type_[0] {
                    if simple == self.schema(&array.items[0]) {
                        return FieldType {
                            typ: format!("Vec<{}>", self.expand_type_(&any_of[0]).typ),
                            attributes: vec![format!(
                                r#"with="{}one_or_many""#,
                                self.schemafy_path
                            )],
                            default: true,
                        };
                    }
                }
            }
            return "serde_json::Value".into();
        } else if typ.type_.len() == 2 {
            if typ.type_[0] == SimpleTypes::Null || typ.type_[1] == SimpleTypes::Null {
                let mut ty = typ.clone();
                ty.type_.retain(|x| *x != SimpleTypes::Null);

                FieldType {
                    typ: format!("Option<{}>", self.expand_type_(&ty).typ),
                    attributes: vec![],
                    default: true,
                }
            } else {
                "serde_json::Value".into()
            }
        } else if typ.type_.len() == 1 {
            match typ.type_[0] {
                SimpleTypes::String => {
                    if typ.enum_.as_ref().map_or(false, |e| e.is_empty()) {
                        "serde_json::Value".into()
                    } else {
                        "String".into()
                    }
                }
                SimpleTypes::Integer => "i64".into(),
                SimpleTypes::Boolean => "bool".into(),
                SimpleTypes::Number => "f64".into(),
                // Handle objects defined inline
                SimpleTypes::Object
                    if !typ.properties.is_empty()
                        || typ.additional_properties == Some(Value::Bool(false)) =>
                {
                    let name = format!(
                        "{}{}",
                        self.current_type.to_pascal_case(),
                        self.current_field.to_pascal_case()
                    );
                    let tokens = self.expand_schema(&name, typ);
                    self.types.push((name.clone(), tokens));
                    name.into()
                }
                SimpleTypes::Object => {
                    let prop = match typ.additional_properties {
                        Some(ref props) if props.is_object() => {
                            let prop = serde_json::from_value(props.clone()).unwrap();
                            self.expand_type_(&prop).typ
                        }
                        _ => "serde_json::Value".into(),
                    };
                    let result = format!("::std::collections::BTreeMap<String, {}>", prop);
                    FieldType {
                        typ: result,
                        attributes: Vec::new(),
                        default: typ.default == Some(Value::Object(Default::default())),
                    }
                }
                SimpleTypes::Array => {
                    let item_type = typ.items.get(0).map_or("serde_json::Value".into(), |item| {
                        self.current_type = format!("{}Item", self.current_type);
                        self.expand_type_(item).typ
                    });
                    let r = format!("Vec<{}>", item_type).into();
                    r
                }
                _ => "serde_json::Value".into(),
            }
        } else {
            "serde_json::Value".into()
        }
    }

    fn expand_definitions(&mut self, schema: &Schema) {
        for (name, def) in &schema.definitions {
            let type_decl = self.expand_schema(name, def);
            let definition_tokens = match def.description {
                Some(ref comment) => {
                    let t = make_doc_comment(comment, LINE_LENGTH);
                    quote! {
                        #t
                        #type_decl
                    }
                }
                None => type_decl,
            };
            self.types.push((name.to_string(), definition_tokens));
        }
    }

    fn expand_schema(&mut self, original_name: &str, schema: &Schema) -> TokenStream {
        self.expand_definitions(schema);

        let pascal_case_name = replace_invalid_identifier_chars(&original_name.to_pascal_case());
        self.current_type.clone_from(&pascal_case_name);
        let (fields, default) = {
            let mut field_expander = FieldExpander {
                default: true,
                expander: self,
            };
            let fields = field_expander.expand_fields(original_name, schema);
            (fields, field_expander.default)
        };

        let name = syn::Ident::new(&pascal_case_name, Span::call_site());
        let is_struct =
            !fields.is_empty() || schema.additional_properties == Some(Value::Bool(false));
        let type_decl = if is_struct {
            if default {
                quote! {
                    #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
                    pub struct #name {
                        #(#fields),*
                    }
                }
            } else {
                let r = quote! {
                    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                    pub struct #name {
                        #(#fields),*
                    }
                };

                r
            }
        } else if schema.enum_.as_ref().map_or(false, |e| !e.is_empty()) {
            let mut optional = false;
            let variants = schema
                .enum_
                .as_ref()
                .map_or(&[][..], |v| v)
                .iter()
                .flat_map(|v| match *v {
                    Value::String(ref v) => {
                        let pascal_case_variant = v.to_pascal_case();
                        let variant_name =
                            rename_keyword("", &pascal_case_variant).unwrap_or_else(|| {
                                let v = syn::Ident::new(&pascal_case_variant, Span::call_site());
                                quote!(#v)
                            });
                        Some(if pascal_case_variant == *v {
                            variant_name
                        } else {
                            quote! {
                                #[serde(rename = #v)]
                                #variant_name
                            }
                        })
                    }
                    Value::Null => {
                        optional = true;
                        None
                    }
                    _ => panic!("Expected string for enum got `{}`", v),
                })
                .collect::<Vec<_>>();

            if optional {
                let enum_name = syn::Ident::new(&format!("{}_", name), Span::call_site());
                quote! {
                    pub type #name = Option<#enum_name>;
                    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                    pub enum #enum_name {
                        #(#variants),*
                    }
                }
            } else {
                quote! {
                    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                    pub enum #name {
                        #(#variants),*
                    }
                }
            }
        } else {
            let typ = self
                .expand_type("", true, schema)
                .typ
                .parse::<TokenStream>()
                .unwrap();
            return quote! {
                pub type #name = #typ;
            };
        };

        if name == original_name {
            type_decl
        } else {
            quote! {
                #[serde(rename = #original_name)]
                #type_decl
            }
        }
    }

    fn expand_file_schema_ref(&mut self, abs_file_path: &Path) -> Schema {
        if let Some(existing) = self.resolved_schemas.get(abs_file_path) {
            return existing.clone();
        } else {
            // Resolve the referenced file Schema.
            let json = std::fs::read_to_string(abs_file_path)
                .unwrap_or_else(|err| panic!("Unable to read `{:#?}`: {}", abs_file_path, err));
            let loaded_schema: Schema = serde_json::from_str(&json).expect("JSON parse error");
            let type_name_from_file = Some(Self::type_from_json_file(abs_file_path));
            let mut reffed_file_expander = Expander {
                root_name: type_name_from_file,
                schemafy_path: self.schemafy_path,
                root: &loaded_schema,
                current_field: "".into(),
                current_type: "".into(),
                types: self.types.clone(),
                schema_directory: abs_file_path
                    .parent()
                    .expect(&format!(
                        "Could not detect directory of file: {:#?}",
                        abs_file_path.as_os_str()
                    ))
                    .to_owned(),
                resolved_schemas: self.resolved_schemas.clone(),
            };
            reffed_file_expander.expand_root();
            // Merge data from the reffed file Expander to reduce lookups
            self.types.append(&mut reffed_file_expander.types);
            self.resolved_schemas
                .insert(abs_file_path.to_owned(), loaded_schema.clone());
            for (resolved_schema_path, resolved_schema) in
                reffed_file_expander.resolved_schemas.into_iter()
            {
                self.resolved_schemas
                    .insert(resolved_schema_path, resolved_schema);
            }
            loaded_schema
        }
    }

    fn to_absolute_path(&self, s: &Path) -> PathBuf {
        if s.is_relative() {
            self.schema_directory.join(s)
        } else {
            s.to_owned()
        }
    }

    fn type_from_json_file(p: &Path) -> &str {
        p.file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.trim_end_matches(".json"))
            .expect(&format!("No file name for [{:#?}]", p.as_os_str()))
    }

    fn is_resolved_ref_path(&self, p: &Path) -> bool {
        self.resolved_schemas
            .get(&self.to_absolute_path(p))
            .is_some()
    }

    pub fn expand(&mut self, schema: &Schema) -> TokenStream {
        match self.root_name {
            Some(name) => {
                let schema = self.expand_schema(name, schema);
                self.types.push((name.to_string(), schema));
            }
            None => self.expand_definitions(schema),
        }

        // Dedupe because we may have visited the same file multiple times when de-referencing
        // external files
        let mut deduped_type_defs = HashMap::new();
        for (name, next) in &self.types {
            if let Some(existing) = deduped_type_defs.insert(name, next) {
                let existing_as_string = format!("{}", existing);
                let next_as_string = format!("{}", next);
                if existing_as_string != next_as_string {
                    panic!(
                        "Found two different definitions for {}: [{}] and [{}]",
                        name, existing_as_string, next_as_string
                    );
                }
            };
        }

        let types = deduped_type_defs.iter().map(|t| *t.1);

        quote! {
            #( #types )*
        }
    }

    pub fn expand_root(&mut self) -> TokenStream {
        self.expand(self.root)
    }
}
