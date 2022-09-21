// from https://doc.rust-lang.org/src/core/slice/mod.rs.html#1669-1681

use core::slice;

/// FIXME: Remove once `const_slice_split_at_mut` lands.
pub(crate) trait ConstSplitAtMutExtensions<T> {
  /// Divides one mutable slice into two at an index.
  ///
  /// The first will contain all indices from `[0, mid)` (excluding
  /// the index `mid` itself) and the second will contain all
  /// indices from `[mid, len)` (excluding the index `len` itself).
  ///
  /// # Panics
  ///
  /// Panics if `mid > len`.
  ///
  /// # Examples
  ///
  /// ```ignore
  /// use crate::ConstSplitAtExtensions;
  /// let mut v = [1, 0, 3, 0, 5, 6];
  /// let (left, right) = v.const_split_at_mut(2);
  /// assert_eq!(left, [1, 0]);
  /// assert_eq!(right, [3, 0, 5, 6]);
  /// left[1] = 2;
  /// right[1] = 4;
  /// assert_eq!(v, [1, 2, 3, 4, 5, 6]);
  /// ```
  #[must_use]
  fn const_split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]);
  /// Divides one mutable slice into two at an index, without doing bounds checking.
  ///
  /// The first will contain all indices from `[0, mid)` (excluding
  /// the index `mid` itself) and the second will contain all
  /// indices from `[mid, len)` (excluding the index `len` itself).
  ///
  /// For a safe alternative see [`split_at_mut`].
  ///
  /// # Safety
  ///
  /// Calling this method with an out-of-bounds index is *[undefined behavior]*
  /// even if the resulting reference is not used. The caller has to ensure that
  /// `0 <= mid <= self.len()`.
  ///
  /// [`split_at_mut`]: slice::split_at_mut
  /// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
  ///
  /// # Examples
  ///
  /// ```ignore
  /// use crate::ConstSplitAtExtensions;
  /// #![feature(slice_split_at_unchecked)]
  ///
  /// let mut v = [1, 0, 3, 0, 5, 6];
  /// // scoped to restrict the lifetime of the borrows
  /// unsafe {
  ///     let (left, right) = v.const_split_at_mut_unchecked(2);
  ///     assert_eq!(left, [1, 0]);
  ///     assert_eq!(right, [3, 0, 5, 6]);
  ///     left[1] = 2;
  ///     right[1] = 4;
  /// }
  /// assert_eq!(v, [1, 2, 3, 4, 5, 6]);
  /// ```
  #[must_use]
  unsafe fn const_split_at_mut_unchecked(&mut self, mid: usize) -> (&mut [T], &mut [T]);
}

impl<T> const ConstSplitAtMutExtensions<T> for [T] {
  #[inline]
  #[track_caller]
  fn const_split_at_mut(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1583
    assert!(mid <= self.len());
    // SAFETY: `[ptr; mid]` and `[mid; len]` are inside `self`, which
    // fulfils the requirements of `from_raw_parts_mut`.
    unsafe { ConstSplitAtMutExtensions::const_split_at_mut_unchecked(self, mid) }
  }

  #[inline]
  unsafe fn const_split_at_mut_unchecked(&mut self, mid: usize) -> (&mut [T], &mut [T]) {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#1684
    let len = self.len();
    let ptr = self.as_mut_ptr();

    // SAFETY: Caller has to check that `0 <= mid <= self.len()`.
    //
    // `[ptr; mid]` and `[mid; len]` are not overlapping, so returning a mutable reference
    // is fine.
    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
      )
    }
  }
}
