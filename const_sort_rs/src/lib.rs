#![no_std]
#![warn(missing_docs)] // TODO deny
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![feature(const_trait_impl)]
#![cfg_attr(feature = "mut_refs", feature(mut_refs))]

#[cfg(feature = "alloc")]
extern crate alloc;

mod const_sort;
pub use const_sort::*;

#[cfg(feature = "mut_refs")]
mod mut_ref_sort;
#[cfg(feature = "mut_refs")]
pub use mut_ref_sort::*;

#[cfg(test)]
mod test;
