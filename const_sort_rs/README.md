# const_sort_rs

[![docs.rs](https://docs.rs/const_sort_rs/badge.svg)](https://docs.rs/const_sort_rs)
[![crates.io](https://img.shields.io/crates/v/const_sort_rs.svg)](https://crates.io/crates/const_sort_rs)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes straight from the crate docs from the source. -->

# THIS CRATE IS BROKEN - Archive Note

The last tested working rust version is `1.71.0-nightly (nightly-2023-04-19)`.
Most const traits have been removed from the standard library.
Once they are back this crate might make a return if it is still needed.

## Requirements

This crate requires a nightly compiler.

## What can this crate do?

This crate implements the [`sort_unstable*`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable) functions and as a bonus exposes a const version of `sort_internals`.
Check out the `ConstSliceSortExt` trait to see all available functions and const examples.

Your types must implement `~const PartialOrd`.

## Example

```rust
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]
use const_sort_rs::ConstSliceSortExt;

const V: [isize; 5] = {
  let mut x = [-5, 4, 1, -3, 2];
  x.const_sort_unstable();
  x
};
assert_eq!(V, [-5, -3, 1, 2, 4])
```

## Authors

[raldone01](https://github.com/raldone01) and [onestacked](https://github.com/chriss0612) are the primary authors and maintainers of this library.

## License

This project is released under either:

- [MIT License](https://github.com/ink-feather-org/const_sort_rs/blob/main/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/ink-feather-org/const_sort_rs/blob/main/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
