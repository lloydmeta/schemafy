## High level flow

This docs seeks to document how this library works

### Entry point,

```rust
schemafy::schemafy!(
 "schemafy_lib/src/schema.json"
)
```

This proc macro passes the tokens inside it to `GenerateBuilder#build_tokens`,
which produces tokens (Rust AST) in the form of `TokenStream`; in other words, code.

#### `build_tokens`

This method parses out the file name and options. It then _reads_ the JSON file, 
deserialising it to a `Schema` (typed version of JSON Schema), and uses it to create
an `Expander`.

Then `Expander#expand` is called, which returns a `TokenStream.

### Expander

#### `expand`

If there is a "root name" on the current `Expander`, `expand_schema` is called to get
the `TokenStream` (Rust AST) of it, which is then added as a type with the "root_name" to the `Expander`'s 
`types`.

If there is no "root name", `expand_definitions` is called on `Expander`: this also fills
in `types`.

After that, the discovered `types` taken, and added to `types` and sliced into a `TokenStream`.

#### `expand_definitions`

This method goes through the `definitions` of a Schema, and calls `expand_schema` on each on
using the _current_ `Expander`, which returns a `TokenStream`. The comments for each definition is
added on top of the type definition, and added to `Expander#types`.

#### `expand_schema`

Takes original name and `Schema`

1. Call `expand_definitions` on the current schema with the current `Expander`
2. Replace the name of the current Expander's type with something valid if needed, binding as `pascal_case_name`
3. Create a `FieldExpander`, passing in the current `Expander`, and call `expand_fields` on it
   passing in the current `Schema`, binding the result to `field`s and `field_expander.default`
4. Defines the AST name of the schema using `pascal_case_name`
5. Defines `is_struct` as whether `fields` are empty (from expander), or whether `additionalProperties` is false
6. Binds a `type_decl` of type `TokenStream`:
    1. If `is_struct`, the returned `TokenStream` is defined as an AST based on `pub struct` with all fields
      * `Default` annotation is set accordingly
    2.If the `Schema` is an `enum` (enum is defined)
      * If `schema.enum_names` is not empty: Goes through the `schema.enum_names` and matches them with `schema.enum` (values), and defines
        an as stream of possible choices `$name: $value`
      * Else spits out enum variants as String
      * Does a bunch more enum stuff.
    3. ELSE
      * Expand the type using `expand_type` (to get a `FieldType`), and define an AST using a type alias
      * This assumes `typ` as a simple-enough AST, I suppose... like mayb
7. If the original name == safe name in 2, return a simple AST of `type_decl`, otherwise return an AST
   with serde rename used.
   
   
   
### `FieldExpander#expand_fields`

A `FieldExpander` is attached to a parent `Expander`

#### `FieldExpander#expand_fields`

Returns `TokenStream`s for each property in the Schema as a `Vec<TokenStream>`

1. Call `self.expander.expand(schema)` to get the Schema that we need to work with (in case it is referenced).
2. For each `properties` on the schema:
  1. Clone it's name and set `expander.current_field` to its name
  2. Get the name of the field
  3. Get if it is required (by checking if it's listed in the Schema's `required` list)
  4. Expand the field type using `expander.expand_type`, passing in the `Schema` for the type, bind it to `field_type`
  5. If `field_type#typ` starts with `Option`, set  `FieldExpander.default` to false
  6. Parse the TokenStream for the field by parsing `field_type#typ`, bind it to `typ`
  6. If the field is a default, create tokens for Serde default annotation
  7. Create tokens for further attribute annotations based on attributes
  7. Create tokens for comments
  8. Create an AST for the field
  
#### `Expander#expand_type`

Calls `Expander#expand_type_`, 

#### `Expander#expand_type_`

`typ` is a Schema

1. If tye typ is a reference to another type (`ref` is filled), simply return the name
2. If the typ is a union type (`any_of`) is filled where there are 2 types
   1. Gets a reference of `any_of`
   2. Passes the first `any_of` to `Expander#schema`, bind it to `simple`
   2. Passes the second `any_of` to `Expander#schema`, bind it to `array`
   3. If `array`'s `type` is not empty, and the first item is `Array`, return a Vector type for `FieldType`
      * otherwise return FieldType of JSON
3. If the typ has a `type` length of 2
   1. Either are `Null`, return an Option type of the other type (not null)
      * Otherwise, return a JSON type
4. If the typ has a `type` length of 1
      * Match type, if they're a simple primitive, that's the FieldType
      * If it's an `Object` type,    
        * Go through `typ`'s `additional_properties`, bind to `prop`
            1. If `prop` is an (JSON) object,
                  * Try to deserialise it as `Schema`, bind 
                    * Call `expand_typ` on it, and return its `typ` String
              *   Otherwise, set prop` to a Serde JSON type
            2. Set `result` to `BTreeMap` of String -> `prop`
            3. Return that as `Field Type`
      * If it's an `Array`
         * Try to get the first item in type's `items` (default to JSON value if it doesn't exist), and set it 
           to `Expander#expand_type_`'s return value (typ), binding the result to `item_type`
         * Return a FieldType of `Vec<$item_type>`
      * Default: fallback to JSON value
5. Default: fallback to JSON value

#### `Expander#schema`

Takes a Schema and returns a referenced Schema

1. If the schema references something else (`_ref` is filled), passes it to `Expander#schema_ref` otherwise
   uses the passed in Schema
2. Goes through the `Schema`'s `all_of` (Schema array)
  * If it is empty, return the current `Schema`
  * If it is defined, _merge_ all of the Schemas, and return it, by iterating and sending to `merge_all_of`
  
#### `merge_all_of`  

Takes a mutable `result` Schema and a reference Schema `r`

1. Goes through all the referenced Schema `properties` as (`k`, and `v`) and:
  * If `k` does not exist in `result` Schema, put it in
  * If `k` *does* exist (e.g. as `entry`), call `merge_all_of`, passing in `entry` and `v`
2. If `r` has a `ref`, clone it to `result`
3. If `r` has a `description`, clone it to `result`
4.  Merge `required` properties of `r` into `result`'s `required` properties
5. Make it so that `result`'s `type_` (Vec<SimpleType>`) only retains items that `r`'s type has


#### `Expander#schema_ref`

Tries to return a resolved Schema