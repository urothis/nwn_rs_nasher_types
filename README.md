# rs_nwn_nasher_types
![docs.rs](https://img.shields.io/docsrs/nwn_nasher_types)
![crates.io](https://img.shields.io/crates/v/nwn_nasher_types.svg)
![Crates.io (latest)](https://img.shields.io/crates/dv/nwn_nasher_types)

A library that provides serialization of Neverwinter Nights module json files generated via [Nasher](https://github.com/squattingmonk/nasher)
This should allow for the creation of a Rust based module tooling.

## Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
nwn-nasher-types = "0.2"
```
## Example
```rust
use nwn_nasher_types::*;
use serde_json::*;
use std::fs;
fn main() {
  let path = "src/module.ifo";
  let file = NwType::from_file_path(path).expect("Failed to open file");
  match nw {
    Ok(value) => {
      println!("Value: {:?}", value);
    }
    Err(e) => {
      panic!("Failed to deserialize {:?}: {}", path, e);
    }
  }
}
```
