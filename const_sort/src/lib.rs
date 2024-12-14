#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![allow(clippy::items_after_statements)]
#![allow(incomplete_features)] // const_closures
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
#![feature(const_slice_split_at_mut)] // const_sort_core
#![feature(const_maybe_uninit_write)] // const_sort_core
#![feature(core_intrinsics)] // const_sort_core
#![feature(const_eval_select)] // const_sort_core
#![feature(const_closures)] // const_sort_core
#![feature(const_slice_index)] // const_sort_core
#![feature(const_cmp)] // const_sort_core
#![feature(unboxed_closures)] // const_slice_sort_ext
#![feature(fn_traits)] // const_slice_sort_ext
// For tests
#![feature(is_sorted)]
#![doc = include_str!("../README.md")]

pub(crate) mod fake_usize_ptr;

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
