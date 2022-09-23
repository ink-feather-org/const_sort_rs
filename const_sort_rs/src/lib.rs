#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![allow(clippy::items_after_statements)]
#![feature(const_refs_to_cell)] // const_sort_core
#![feature(const_trait_impl)] // const_sort_core
#![feature(const_num_from_num)] // const_sort_core
#![feature(const_option)] // const_sort_core
#![feature(const_mut_refs)] // const_sort_core
#![feature(const_swap)] // const_sort_core
#![feature(maybe_uninit_slice)] // const_sort_core
#![feature(strict_provenance)] // const_sort_core
#![feature(const_ptr_read)] // const_sort_core
#![feature(const_deref)] // const_sort_core
#![feature(const_reverse)] // const_sort_core
#![feature(mixed_integer_ops)] // const_sort_core
#![feature(const_maybe_uninit_write)] // const_sort_core
#![feature(core_intrinsics)] // const_sort_core
#![feature(const_eval_select)] // const_sort_core
#![feature(const_slice_index)] // const_sort_core
#![feature(const_cmp)] // const_sort_core
#![feature(const_slice_from_raw_parts_mut)] // slice_const_split_at_mut FIXME: Replace with const_slice_split_at_mut once it lands.
#![feature(unboxed_closures)] // const_slice_sort_ext
#![feature(fn_traits)] // const_slice_sort_ext
// For tests
#![feature(is_sorted)]

/*!
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

- [MIT License](https://github.com/raldone01/const_sort_rs/blob/main/LICENSE-MIT)
- [Apache License (Version 2.0)](https://github.com/raldone01/const_sort_rs/blob/main/LICENSE-APACHE)

at your choosing.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

*/

pub(crate) mod fake_usize_ptr;
pub(crate) mod slice_const_split_at_mut;

#[allow(
  clippy::undocumented_unsafe_blocks,
  clippy::identity_op,
  clippy::unnecessary_mut_passed,
  clippy::too_many_lines,
  clippy::doc_markdown,
  clippy::cognitive_complexity,
  clippy::cast_possible_truncation
)]
pub mod const_sort;

mod const_slice_sort_ext;
pub use const_slice_sort_ext::ConstSliceSortExt;

#[cfg(test)]
mod test;
