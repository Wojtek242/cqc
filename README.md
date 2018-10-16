CQC
===

[![Latest version](https://img.shields.io/crates/v/cqc.svg)](https://crates.io/crates/cqc)
[![Documentation](https://docs.rs/cqc/badge.svg)](https://docs.rs/cqc)
![License](https://img.shields.io/crates/l/cqc.svg)

A Rust implementation of the [CQC
interface](https://stephaniewehner.github.io/SimulaQron/PreBetaDocs/CQCInterface.html).

- [Documentation](https://docs.rs/cqc)

## Design goals

The following goals drive the design of the `cqc` crate:

- The user should be able to create any valid packet

  This goal is achieved by having correct struct definitions for the different
  CQC headers.

- It should be difficult, though preferably impossible, to create invalid
  packets

  The second goal is achieved by using Rust's typing system as much as
  possible, especially enums for fields with only a small set of possible
  values.

- Decoding should raise errors if unrecognised values are detected

  This is achieved through a combination of type definitions and
  deserialization implementations.

- No assumption about the user's run-time should be made

  The library is sans-io and only provides a very plain encoder and decoder.
  The intention is that the user builds packets using the `cqc` library, but
  I/O is their responsibility.  The `Serialize` and `Deserialize` traits are
  implemented so that the user can simply use `serde` for encode/decode.

## Limitations

Factory and Sequence Headers are not currently fully supported.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cqc = "0.3"
```

and this to your source file:

```rust
extern crate cqc;
```
