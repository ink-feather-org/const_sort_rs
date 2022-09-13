const_sort_rs
=========================

[![Rust-CI](https://github.com/raldone01/const_sort_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/raldone01/const_sort_rs/actions/workflows/rust.yml)
[![docs.rs](https://docs.rs/const_sort_rs/badge.svg)](https://docs.rs/const_sort_rs)
[![crates.io](https://img.shields.io/crates/v/const_sort_rs.svg)](https://crates.io/crates/const_sort_rs)
[![rustc](https://img.shields.io/badge/rustc-nightly-lightgrey)](https://doc.rust-lang.org/nightly/std/)

<!-- The rest of this section comes straight from the crate docs from the source. -->

Requirements
------------

This crate requires a nightly compiler.

What can this crate do?
------------------------

This crate implements the [`sort_unstable`](https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort_unstable) functions and the sort_internals

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
