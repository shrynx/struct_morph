# Struct Morph [![Rust](https://github.com/shrynx/struct_morph/actions/workflows/rust.yml/badge.svg)](https://github.com/shrynx/struct_morph/actions/workflows/rust.yml) [![Version](https://img.shields.io/crates/v/struct_morph.svg)](https://crates.io/crates/struct_morph)

macro for morphing one struct into another.

## Installation

```sh
cargo add struct_morph
```

or manually

```toml
struct_morph = "0.6"
```

## Usage

I occasionally run into use case where i have two structs representing very similar data. They share most of the fields or can sometmes even be subset of one.
Say we have a struct `ProductRow` coming from database and another struct `ProductInfo` which is what will be sent as a json from the api.

```rust
struct ProductRow {
    id: i32,
    name: String,
    description: String,
    available_count: i32,
    base_price: i32,
    discount: i32,
    created_at: DateTime,
    updated_at: DateTime,
}

struct ProductInfo {
    id: i32,
    name: String,
    description: String,
    is_available: bool,
    price: i32,
}
```

now for this small when we need to convert ProductRows to ProductInfos we can manually do it, but for larger structs it becomes quite mechanical.

with this library you can write the following

```rust
use struct_morph::{morph, morph_field};

#[morph(ProductRow)]
struct ProductInfo {
    id: i32,
    #[morph_field(select = name)]
    title: String,
    description: String,
    #[morph_field(transform = "is_available")]
    is_available: bool,
    #[morph_field(transform = "net_price")]
    price: i32,
}

fn is_available(value: &ProductRow) -> bool {
    value.available_count > 0
}

fn net_price(value: &ProductRow) -> i32 {
    value.base_price - value.discount
}
```

and then you can simply generate a product info from a product row

```rust
let product_row: ProductRow = ProductRow {
    id: 10,
    name: "The Rust Programming Language".to_string(),
    description: "The official book on the Rust programming language".to_string(),
    available_count: 10,
    base_price: 50,
    discount: 10,
    created_at: DateTime::now(),
    updated_at: DateTime::now(),
};

let product_info: ProductInfo = ProductInfo::from(product_row);
```

This will copy the values for fields with same name (and type) and for the rest one can provide customer transform functions.
It does so by implementing a `From` trait for the source and target struct.

There are 2 types field modifiers 

- transform

```rust
#[morph_field(transform = "transform_func")]
```
this takes an transform function which takes &SourceStruct as a param and returns the correct type

- select

```rust
#[morph_field(select = source_field)]
```
this takes a source field to replace the value for the field
