//! TODO

#![no_std]
#![warn(missing_docs)] // TODO deny
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_num_from_num)]
#![feature(const_option)]
#![feature(const_result_drop)]
#![feature(const_mut_refs)]
#![feature(const_swap)]
#![feature(const_cmp)] // For tests

#[cfg(feature = "alloc")]
extern crate alloc;

mod const_sort;
pub use const_sort::*;

#[cfg(test)]
mod test;
