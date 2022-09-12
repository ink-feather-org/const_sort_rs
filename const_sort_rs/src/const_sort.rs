pub trait ConstSortable {
  fn const_sort(self, low: usize, high: usize) -> Self;
}

// Not possible because T might contain interior mutability
// impl<T: Copy + ~const PartialOrd, const N: usize> const ConstSortable for [T; N]
#[macro_export]
macro_rules! impl_static_sorter {
  ($type:ty) => {};
}

/// TODO: move into impl_static_sorter macro
impl<const N: usize> const ConstSortable for [u64; N] {
  fn const_sort(mut self, low: usize, high: usize) -> Self {
    debug_assert!(high < N);

    let mut low = isize::try_from(low).ok().unwrap();
    let mut high = isize::try_from(high).ok().unwrap();

    // TODO: change to this quick sort impl and drop the copy requirement
    // TODO: do todo in lib.rs (remove copy)
    // https://www.hackertouch.com/quick-sort-in-rust.html

    let range = high - low;
    if range <= 0 || range >= isize::try_from(N).ok().unwrap() {
      return self;
    }

    loop {
      let mut i = low;
      let mut j = high;
      // Copy required here (clone does not allow destructors) (also can't borrow p because its index may change with the swap)
      let p = self[(low + ((high - low) >> 1)) as usize];
      loop {
        while self[i as usize] < p {
          i += 1;
        }
        while self[j as usize] > p {
          j -= 1;
        }
        if i <= j {
          if i != j {
            self.swap(i as usize, j as usize);
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
          self = Self::const_sort(self, low as usize, j as usize);
        }
        low = i;
      } else {
        if i < high {
          self = Self::const_sort(self, i as usize, high as usize)
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
