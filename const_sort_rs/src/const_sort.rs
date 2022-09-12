/// Trait for Sorting a Array in a Const Context
pub trait ArrayConstStableSortable {
  /// Sort the array
  fn const_sort(self) -> Self;
}

//pub trait ArrayConstUnstableSortable {
//  fn const_sort_unstable(self, low: usize, high: usize) -> Self;
//}

impl<const N: usize, T: ~const PartialOrd> const ArrayConstStableSortable for [T; N] {
  fn const_sort(self) -> Self {
    self.const_sort_impl(0, N - 1)
  }
}

trait ArrayConstStableSortableImpl {
  fn const_sort_impl(self, low: usize, high: usize) -> Self;
}

impl<const N: usize, T: ~const PartialOrd> const ArrayConstStableSortableImpl for [T; N] {
  fn const_sort_impl(mut self, low: usize, high: usize) -> Self {
    debug_assert!(high < N);

    let mut low = isize::try_from(low).ok().unwrap();
    let mut high = isize::try_from(high).ok().unwrap();

    let range = high - low;
    if range <= 0 || range >= isize::try_from(N).ok().unwrap() {
      return self;
    }

    loop {
      let mut i = low;
      let mut j = high;

      let mut p = low + ((high - low) >> 1);
      loop {
        while self[i as usize] < self[p as usize] {
          i += 1;
        }
        while self[j as usize] > self[p as usize] {
          j -= 1;
        }
        if i <= j {
          if i != j {
            self.swap(i as usize, j as usize);
            if p == i {
              p = j;
            } else if p == j {
              p = i;
            }
          }
          i += 1;
          j -= 1;
        }
        if i > j {
          break;
        }
      }
      if j - low < high - i {
        if low < j {
          self = self.const_sort_impl(low as usize, j as usize);
        }
        low = i;
      } else {
        if i < high {
          self = self.const_sort_impl(i as usize, high as usize)
        }
        high = j;
      }
      if low >= high {
        break;
      }
    }
    self
  }
}
