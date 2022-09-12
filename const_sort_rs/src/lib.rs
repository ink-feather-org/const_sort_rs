#![no_std]
#![warn(missing_docs)] // TODO deny
#![warn(clippy::undocumented_unsafe_blocks, clippy::pedantic, clippy::nursery)]
#![feature(const_trait_impl)]
#![feature(const_num_from_num)]
#![feature(const_option)]
#![feature(const_result_drop)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(ptr_sub_ptr)]
#![feature(const_reverse)]
#![feature(const_ptr_read)]
#![feature(const_deref)]
#![feature(const_intoiterator_identity)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod const_sort;
pub use const_sort::*;

mod mut_ref_sort;
pub use mut_ref_sort::*;

#[cfg(test)]
mod test;
