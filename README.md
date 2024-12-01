# Jupijson

_Jupijson is a Rust library I built from scratch to learn about Rust and JSON._

## Features

- Simple
- Easy to use

## Prerequisites

Ensure you have [Rust installed](https://www.rust-lang.org/tools/install) on your system.

## Adding jupijson to your local project

1. Clone this repository:

```bash
git clone https://github.com/ToDucThanh/jupijson.git
```

2. In your Rust project, add a dependency pointing to the local directory in your Cargo.toml:

```toml
[dependencies]
jupijson = { path = "../path-to-jupijson" }
```

Replace `../path-to-jupijson` with the actual relative path to the library.

3. Use the library in your code:

```rs
use jupijson::{loads, JsonValue};

fn main() {
    let json_str = r#"{"key": "value"}"#;
    match loads(json_str) {
        Ok(user) => {
            if let JsonValue::Object(obj) = user {
                println!("{:?}", obj);
            }
        }
        Err(e) => eprintln!("Error parsing JSON: {:?}", e),
    }
}
```

## Running examples

```bash
cargo run --example ex
```

The result:

```bash
Name: To Duc Thanh
Nationality: Viet Nam
Age: 24
Skills:
  - Python
  - JavaScript
  - SQL
  - Rust
Address:
  City: Ho Chi Minh
  ZIP: 70000
```
