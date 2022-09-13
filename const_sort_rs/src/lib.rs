//! TODO

#![no_std]
//#![warn(missing_docs)] // TODO deny
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_num_from_num)]
#![feature(const_option)]
#![feature(const_result_drop)]
#![feature(const_mut_refs)]
#![feature(const_swap)]
#![feature(const_cmp)] // For tests
#![feature(const_slice_index)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(maybe_uninit_slice)] // const_sort_core
#![feature(strict_provenance)] // const_sort_core
//#![feature(const_slice_index)] // const_sort_core
#![feature(const_ptr_read)] // const_sort_core
#![feature(const_deref)] // const_sort_core
#![feature(const_reverse)] // const_sort_core
#![feature(mixed_integer_ops)] // const_sort_core
#![feature(const_maybe_uninit_write)] // const_sort_core
#![feature(is_sorted)] // For tests

mod const_sort;
pub use const_sort::*;

pub(crate) mod slice_const_split_at;

mod const_sort_core;
pub use const_sort_core::*;

#[cfg(test)]
mod test;
