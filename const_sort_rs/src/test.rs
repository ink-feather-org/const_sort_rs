extern crate alloc;
use alloc::vec::Vec;
use rand::{prelude::StdRng, Rng, SeedableRng};
const RAND_CNT: usize = 10_000;
pub use super::*;

fn gen_array(n: usize) -> Vec<u32> {
  let mut rng = StdRng::seed_from_u64(69420);
  (0..n).map(|_| rng.gen()).collect()
}

#[test]
fn const_slice_sort() {
  const ARR: [u8; 4] = {
    let mut x = [2, 3, 5, 4];
    x.const_sort_unstable();
    x
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_slice_sort_rng() {
  let mut vec = gen_array(RAND_CNT);
  vec.const_sort_unstable();
  assert!(vec.is_sorted())
}

#[test]
fn const_core_slice_heapsort() {
  const ARR: [u8; 4] = {
    let mut x = [2, 3, 5, 4];
    x.const_heapsort();
    x
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_core_slice_heapsort_rng() {
  let mut vec = gen_array(RAND_CNT);
  vec.const_heapsort();
  assert!(vec.is_sorted())
}

#[test]
fn const_core_slice_quicksort() {
  const ARR: [u8; 4] = {
    let mut x = [2, 3, 5, 4];
    x.const_quicksort();
    x
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_core_slice_quicksort_rng() {
  let mut vec = gen_array(RAND_CNT);
  vec.const_quicksort();
  assert!(vec.is_sorted())
}
