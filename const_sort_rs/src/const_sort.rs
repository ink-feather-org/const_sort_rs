use crate::utils::split_at_mut_unchecked;
/// Trait for Sorting a Array in a Const Context
pub trait ArrayConstUnstableSortable {
  /// Sort the array
  fn const_sort_unstable(self) -> Self;
}

impl<const N: usize, T: ~const PartialOrd + Ord> const ArrayConstUnstableSortable for [T; N] {
  fn const_sort_unstable(mut self) -> Self {
    (&mut self[..]).const_sort_unstable();
    self
  }
}

/// Trait for sorting slices in a const context
pub trait SliceConstUnstableSortable {
  /// Sort the slice
  fn const_sort_unstable(&mut self);
}

const fn partition<T: ~const PartialOrd + Ord>(inp: &mut [T]) -> (&mut [T], &mut [T]) {
  assert!(inp.len() > 1);
  let mut p = inp.len() / 2;
  let mut i = 0;
  let mut j = inp.len() - 1;
  loop {
    while inp[i] < inp[p] {
      i += 1;
    }
    while inp[j] > inp[p] {
      j -= 1;
    }
    if i == j {
      break;
    }
    inp.swap(i, j);
    if p == i {
      p = j;
    } else if p == j {
      p = i;
    }
  }
  unsafe { split_at_mut_unchecked(inp, p) }
}

impl<T: ~const PartialOrd + Ord> const SliceConstUnstableSortable for [T] {
  fn const_sort_unstable(&mut self) {
    if self.len() <= 1 {
      return;
    }
    let (left, right) = partition(self);
    left.const_sort_unstable();
    right.const_sort_unstable();
  }
}

trait SliceConstStableSortableImpl {
  fn const_sort_impl(&mut self, low: usize, high: usize);
}
