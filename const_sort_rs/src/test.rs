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
    let mut v = [2, 3, 5, 4];
    const_heapsort(&mut v, const_pred_lt);
    v
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_core_slice_heapsort_rng() {
  let mut v = gen_array(RAND_CNT);
  const_heapsort(&mut v, const_pred_lt);
  assert!(v.is_sorted())
}

#[test]
fn const_core_slice_quicksort() {
  const ARR: [u8; 4] = {
    let mut v = [2, 3, 5, 4];
    const_quicksort(&mut v, const_pred_lt);
    v
  };
  assert_eq!(&ARR, &[2, 3, 4, 5])
}
#[test]
fn const_core_slice_quicksort_rng() {
  let mut v = gen_array(RAND_CNT);
  const_quicksort(&mut v, const_pred_lt);
  assert!(v.is_sorted())
}

#[test]
fn const_core_slice_sort_unstable() {
  let mut v = gen_array(RAND_CNT);
  v.const_sort_unstable();
  assert!(v.is_sorted())
}

#[test]
fn const_core_slice_sort_unstable_by() {
  let mut v = gen_array(RAND_CNT);
  v.const_sort_unstable_by(|a, b| a.cmp(b));
  assert!(v.is_sorted());
}

#[test]
fn const_core_slice_sort_unstable_by_key() {
  let mut v = [-5i32, 4, 1, -3, 2];
  v.sort_unstable_by_key(|k| k.abs());
  assert_eq!(v, [1, 2, -3, 4, -5]);
}

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn rustc_sort_unstable() {
  use core::cmp::Ordering::{Equal, Greater, Less};
  use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

  // Miri is too slow (but still need to `chain` to make the types match)
  let lens = if cfg!(miri) {
    (2..20).chain(0..0)
  } else {
    (2..25).chain(500..510)
  };
  let rounds = if cfg!(miri) { 1 } else { 100 };

  let mut v = [0; 600];
  let mut tmp = [0; 600];
  let mut rng = StdRng::from_entropy();

  for len in lens {
    let v = &mut v[0..len];
    let tmp = &mut tmp[0..len];

    for &modulus in &[5, 10, 100, 1000] {
      for _ in 0..rounds {
        for i in 0..len {
          v[i] = rng.gen::<i32>() % modulus;
        }

        // Sort in default order.
        tmp.copy_from_slice(v);
        tmp.const_sort_unstable();
        assert!(tmp.windows(2).all(|w| w[0] <= w[1]));

        // Sort in ascending order.
        tmp.copy_from_slice(v);
        tmp.const_sort_unstable_by(|a, b| a.cmp(b));
        assert!(tmp.windows(2).all(|w| w[0] <= w[1]));

        // Sort in descending order.
        tmp.copy_from_slice(v);
        tmp.const_sort_unstable_by(|a, b| b.cmp(a));
        assert!(tmp.windows(2).all(|w| w[0] >= w[1]));

        // Test heapsort using `<` operator.
        tmp.copy_from_slice(v);
        const_heapsort(tmp, |a, b| a < b);
        assert!(tmp.windows(2).all(|w| w[0] <= w[1]));

        // Test heapsort using `>` operator.
        tmp.copy_from_slice(v);
        const_heapsort(tmp, |a, b| a > b);
        assert!(tmp.windows(2).all(|w| w[0] >= w[1]));
      }
    }
  }

  // Sort using a completely random comparison function.
  // This will reorder the elements *somehow*, but won't panic.
  for i in 0..v.len() {
    v[i] = i as i32;
  }
  v.sort_unstable_by(|_, _| *[Less, Equal, Greater].choose(&mut rng).unwrap());
  v.sort_unstable();
  for i in 0..v.len() {
    assert_eq!(v[i], i as i32);
  }

  // Should not panic.
  [0i32; 0].sort_unstable();
  [(); 10].sort_unstable();
  [(); 100].sort_unstable();

  let mut v = [0xDEADBEEFu64];
  v.sort_unstable();
  assert!(v == [0xDEADBEEF]);
}

#[test]
fn tmp_doc_test() {}
