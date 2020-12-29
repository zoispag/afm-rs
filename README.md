# afm-rs

<a href="https://crates.io/crates/afm-rs"><img src="https://img.shields.io/crates/v/afm-rs.svg" /></a>
![CI](https://github.com/zoispag/afm-rs/workflows/CI/badge.svg)

A validator for greek tax identification number (ΑΦΜ)

## Usage

Add `afm-rs` under `[dependencies]` in your `Cargo.toml`:

```toml
[dependencies]
afm-rs = "0.1.0"
```

Use the validator:

```rust
use afm-rs;

// An invalid AFM
let (is_valid, err) = afm::validate("123456789");
assert!(!is_valid);
println!("{}", err);

// A valid AFM
let (is_valid, err) = afm::validate("997788278");
assert!(is_valid);
assert_eq!("", err)
```
