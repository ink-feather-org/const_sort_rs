extern crate alloc;
use alloc::vec::Vec;
use rand::{prelude::StdRng, Rng, SeedableRng};
const RAND_CNT: usize = 10_000;
pub use crate::const_sort::{const_heapsort, const_quicksort};
use crate::{const_pred_lt, ConstSliceSortExt};

fn gen_array(n: usize) -> Vec<u32> {
  let mut rng = StdRng::seed_from_u64(69420);
  (0..n).map(|_| rng.gen()).collect()
}

#[test]
fn const_core_slice_heapsort() {
  const ARR: [u8; 4] = {
    let mut x = [2, 3, 5, 4];
    const_heapsort(&mut x, &mut const_pred_lt);
    x
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_core_slice_heapsort_rng() {
  let mut vec = gen_array(RAND_CNT);
  const_heapsort(&mut vec, &mut const_pred_lt);
  assert!(vec.is_sorted())
}

#[test]
fn const_core_slice_quicksort() {
  const ARR: [u8; 4] = {
    let mut x = [2, 3, 5, 4];
    const_quicksort(&mut x, const_pred_lt);
    x
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_core_slice_quicksort_rng() {
  let mut vec = gen_array(RAND_CNT);
  const_quicksort(&mut vec, const_pred_lt);
  assert!(vec.is_sorted())
}

#[test]
fn const_core_slice_sort_unstable() {
  let mut vec = gen_array(RAND_CNT);
  vec.const_sort_unstable();
  assert!(vec.is_sorted())
}

#[test]
fn const_core_slice_sort_unstable_by() {
  let mut vec = gen_array(RAND_CNT);
  vec.const_sort_unstable_by(|a, b| a.cmp(b));
  assert!(vec.is_sorted());
}

#[test]
fn const_core_slice_sort_unstable_by_key() {
  let mut v = [-5i32, 4, 1, -3, 2];
  v.sort_unstable_by_key(|k| k.abs());
  assert!(v == [1, 2, -3, 4, -5]);
}

#[test]
fn tmp_doc_test() {}
