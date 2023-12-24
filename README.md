[Crate](https://crates.io/crates/openapiv3-extended) | [Github](https://github.com/kurtbuilds/openapiv3-extended)

# OpenAPI v3 Extended

A library to de/serialize OpenAPI specifications. It offers:

- Convenience methods for creating, modifying, and merging specs
- A simple API, prioritizing usability without sacrificing correctness
- Support for v2.0 and v3.0 specs. (v3.1 is not currently supported nor being actively developed. PRs are welcome though.)

# Installation

```toml
[dependencies]
openapiv3-extended = ".."
```

Note the library is named `openapiv3` and imported as `use openapiv3;`, despite the package name being `openapiv3-extended`.

# OpenAPI v3 ![example workflow](https://github.com/glademiller/openapiv3/actions/workflows/rust.yml/badge.svg)

This crate provides data structures that represent the [OpenAPI v3.0.x specification](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/3.0.3.md).
Note this does not cover OpenAPI v3.1 (yet) which was an incompatible change.

# Usage

Here is a basic example: 

```rust
use serde_json;
use openapiv3::OpenAPI;

fn main() {
    let data = include_str!("openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).unwrap();
    println!("{:?}", openapi);
}
```

You can use this crate to upgrade a Swagger 2.0 spec to OpenAPI 3.0.x. To support v2, enable the `v2`  feature.

```rust
// [dependencies]
// openapiv3-extended = { version = "..", features = ["v2"] }
use openapiv3::VersionedOpenAPI;

fn main() {
    let data = include_str!("swagger.json");
    let openapi: VersionedOpenAPI = serde_json::from_str(data).unwrap();
    println!("{:?}", openapi);
    let openapi: OpenAPI = openapi.upgrade(); // version 3.0
    println!("{:?}", openapi);
}
```

## Acknowledgements

This library started as a fork of https://github.com/glademiller/openapiv3. Both libraries support full de/ser of OpenAPI v3.0 specs. This fork offers:

- many convenience methods for creating, modifying, and merging specs
- support for OpenAPI v2.0, and upgrading v2.0 specs to v3.0
- a simplified API, where we aim to prioritize usability without sacrificing correctness

## Goals
* Provide a deserialization for the specification that maps cleanly to Rust enums etc.

## Non Goals
* Deserialization and subsequent re-serialization are 100% the same.
    * Some defaults show-up when serializing that may not have existed in the input.

## Issues
Schemas without a type will end up as any data type as per the specification and can have any parameters of any schema type. Some Open API documents don't include the type parameter it would be nice to try to derive the type but the crate as of right now meets my needs.