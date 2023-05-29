# rs_nwn_nasher_types

[![docs.rs](https://img.shields.io/docsrs/nwn_nasher_types)](https://docs.rs/nwn_nasher_types/latest/)
[![crates.io](https://img.shields.io/crates/v/nwn_nasher_types.svg)](https://crates.io/crates/nwn_nasher_types)
[![Crates.io (latest)](https://img.shields.io/crates/dv/nwn_nasher_types)](https://crates.io/crates/nwn_nasher_types/versions)

A library that provides serialization of Neverwinter Nights json files generated via [Nasher](https://github.com/squattingmonk/nasher)

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

fn main() {
  let path = "src/module.ifo";
  let mw = NwType::from_file_path(path).expect("Failed to open file");
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
