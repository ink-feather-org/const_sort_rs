extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use rand::{prelude::StdRng, Rng, SeedableRng};

pub use crate::const_sort::{const_heapsort, const_quicksort};
use crate::{const_pred_lt, ConstSliceSortExt};

const RAND_CNT: usize = 10_000;

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

mod from_rustc {
  use super::*;

  #[test]
  #[cfg(not(target_arch = "wasm32"))]
  fn sort_unstable() {
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
    v.const_sort_unstable_by(|_, _| *[Less, Equal, Greater].choose(&mut rng).unwrap());
    v.const_sort_unstable();
    for i in 0..v.len() {
      assert_eq!(v[i], i as i32);
    }

    // Should not panic.
    [0i32; 0].const_sort_unstable();
    [(); 10].const_sort_unstable();
    [(); 100].const_sort_unstable();

    let mut v = [0xDEADBEEFu64];
    v.const_sort_unstable();
    assert!(v == [0xDEADBEEF]);
  }

  #[test]
  #[cfg(not(target_arch = "wasm32"))]
  #[cfg_attr(miri, ignore)] // Miri is too slow
  fn select_nth_unstable() {
    use core::cmp::Ordering::{Equal, Greater, Less};
    use rand::rngs::StdRng;
    use rand::seq::SliceRandom;
    use rand::{Rng, SeedableRng};

    let mut rng = StdRng::from_entropy();

    for len in (2..21).chain(500..501) {
      let mut orig = vec![0; len];

      for &modulus in &[5, 10, 1000] {
        for _ in 0..10 {
          for i in 0..len {
            orig[i] = rng.gen::<i32>() % modulus;
          }

          let v_sorted = {
            let mut v = orig.clone();
            v.sort();
            v
          };

          // Sort in default order.
          for pivot in 0..len {
            let mut v = orig.clone();
            v.const_select_nth_unstable(pivot);

            assert_eq!(v_sorted[pivot], v[pivot]);
            for i in 0..pivot {
              for j in pivot..len {
                assert!(v[i] <= v[j]);
              }
            }
          }

          // Sort in ascending order.
          for pivot in 0..len {
            let mut v = orig.clone();
            let (left, pivot, right) = v.const_select_nth_unstable_by(pivot, |a, b| a.cmp(b));

            assert_eq!(left.len() + right.len(), len - 1);

            for l in left {
              assert!(l <= pivot);
              for r in right.iter_mut() {
                assert!(l <= r);
                assert!(pivot <= r);
              }
            }
          }

          // Sort in descending order.
          let sort_descending_comparator = |a: &i32, b: &i32| b.cmp(a);
          let v_sorted_descending = {
            let mut v = orig.clone();
            v.sort_by(sort_descending_comparator);
            v
          };

          for pivot in 0..len {
            let mut v = orig.clone();
            v.const_select_nth_unstable_by(pivot, sort_descending_comparator);

            assert_eq!(v_sorted_descending[pivot], v[pivot]);
            for i in 0..pivot {
              for j in pivot..len {
                assert!(v[j] <= v[i]);
              }
            }
          }
        }
      }
    }

    // Sort at index using a completely random comparison function.
    // This will reorder the elements *somehow*, but won't panic.
    let mut v = [0; 500];
    for i in 0..v.len() {
      v[i] = i as i32;
    }

    for pivot in 0..v.len() {
      v.const_select_nth_unstable_by(pivot, |_, _| {
        *[Less, Equal, Greater].choose(&mut rng).unwrap()
      });
      v.sort();
      for i in 0..v.len() {
        assert_eq!(v[i], i as i32);
      }
    }

    // Should not panic.
    [(); 10].const_select_nth_unstable(0);
    [(); 10].const_select_nth_unstable(5);
    [(); 10].const_select_nth_unstable(9);
    [(); 100].const_select_nth_unstable(0);
    [(); 100].const_select_nth_unstable(50);
    [(); 100].const_select_nth_unstable(99);

    let mut v = [0xDEADBEEFu64];
    v.const_select_nth_unstable(0);
    assert!(v == [0xDEADBEEF]);
  }

  #[test]
  #[should_panic(expected = "index 0 greater than length of slice")]
  fn const_select_nth_unstable_zero_length() {
    [0i32; 0].const_select_nth_unstable(0);
  }

  #[test]
  #[should_panic(expected = "index 20 greater than length of slice")]
  fn const_select_nth_unstable_past_length() {
    [0i32; 10].const_select_nth_unstable(20);
  }

  #[test]
  fn test_const_is_sorted() {
    let empty: [i32; 0] = [];

    assert!([1, 2, 2, 9].const_is_sorted());
    assert!(![1, 3, 2].const_is_sorted());
    assert!([0].const_is_sorted());
    assert!(empty.const_is_sorted());
    assert!(![0.0, 1.0, f32::NAN].const_is_sorted());
    assert!([-2, -1, 0, 3].const_is_sorted());
    assert!(![-2i32, -1, 0, 3].const_is_sorted_by_key(|n| n.abs()));
    assert!(!["c", "bb", "aaa"].const_is_sorted());
    assert!(["c", "bb", "aaa"].const_is_sorted_by_key(|s| s.len()));
  }
}

mod const_rustc {
  // TODO: port tinyrand to const
}

#[test]
fn tmp_doc_test() {}
