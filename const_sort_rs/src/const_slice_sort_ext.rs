use core::{cmp::Ordering, marker::Destruct};

use crate::const_sort;

#[const_trait]
/// Trait for sorting slices in const items.
pub trait ConstSliceSortExt<T> {
  /// Sorts the slice, but might not preserve the order of equal elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on slices with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behaviour.
  ///
  /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
  /// slice consists of several concatenated sorted sequences.
  ///
  /// # Examples
  ///
  /// ```rust
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const V: [isize; 5] = {
  ///   let mut v = [-5, 4, 1, -3, 2];
  ///   v.const_sort_unstable();
  ///   v
  /// };
  /// assert_eq!(V, [-5, -3, 1, 2, 4])
  /// ```
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  fn const_sort_unstable(&mut self)
  where
    T: Ord;
  /// Sorts the slice with a comparator function, but might not preserve the order of equal
  /// elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(*n* \* log(*n*)) worst-case.
  ///
  /// The comparator function must define a total ordering for the elements in the slice. If
  /// the ordering is not total, the order of the elements is unspecified. An order is a
  /// total order if it is (for all `a`, `b` and `c`):
  ///
  /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
  /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
  ///
  /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
  /// `partial_cmp` as our sort function when we know the slice doesn't contain a `NaN`.
  ///
  /// ```rust
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// #![feature(const_cmp)]
  /// #![feature(const_option)]
  /// # use core::cmp::Ordering;
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const FLOATS: [f64; 5] = {
  ///   let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
  ///   // no const closures yet
  ///   const fn pred(a: &f64, b: &f64) -> Ordering {
  ///     a.partial_cmp(b).unwrap()
  ///   }
  ///   floats.const_sort_unstable_by(pred);
  ///   floats
  /// };
  /// assert_eq!(FLOATS, [1.0, 2.0, 3.0, 4.0, 5.0]);
  /// ```
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on slices with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behavior.
  ///
  /// It is typically faster than stable sorting, except in a few special cases, e.g., when the
  /// slice consists of several concatenated sorted sequences.
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// #![feature(const_cmp)]
  /// # use core::cmp::Ordering;
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const V: [i32; 5] = [5, 4, 1, 3, 2];
  ///
  /// const S: [i32; 5] = {
  ///   let mut v = V;
  ///   // no const closures yet
  ///   const fn pred(a: &i32, b: &i32) -> Ordering {
  ///     a.cmp(b)
  ///   }
  ///   v.const_sort_unstable_by(pred);
  ///   v
  /// };
  /// assert_eq!(S, [1, 2, 3, 4, 5]);
  ///
  /// // reverse sorting
  /// const R: [i32; 5] = {
  ///   let mut v = V;
  ///   // no const closures yet
  ///   const fn pred(a: &i32, b: &i32) -> Ordering {
  ///     b.cmp(a)
  ///   }
  ///   v.const_sort_unstable_by(pred);
  ///   v
  /// };
  /// assert_eq!(R, [5, 4, 3, 2, 1]);
  /// ```
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  fn const_sort_unstable_by<F>(&mut self, compare: F)
  where
    F: FnMut(&T, &T) -> Ordering;
  /// Sorts the slice with a key extraction function, but might not preserve the order of equal
  /// elements.
  ///
  /// This sort is unstable (i.e., may reorder equal elements), in-place
  /// (i.e., does not allocate), and *O*(m \* *n* \* log(*n*)) worst-case, where the key function is
  /// *O*(*m*).
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on [pattern-defeating quicksort][pdqsort] by Orson Peters,
  /// which combines the fast average case of randomized quicksort with the fast worst case of
  /// heapsort, while achieving linear time on slices with certain patterns. It uses some
  /// randomization to avoid degenerate cases, but with a fixed seed to always provide
  /// deterministic behaviour.
  ///
  /// Due to its key calling strategy, [`sort_unstable_by_key`](#method.sort_unstable_by_key)
  /// is likely to be slower than [`sort_by_cached_key`](#method.sort_by_cached_key) in
  /// cases where the key function is expensive.
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// #![feature(const_cmp)]
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const V: [i32; 5] = {
  ///   let mut v = [-5i32, 4, 1, -3, 2];
  ///   // no const closures yet
  ///   const fn pred(k: &i32) -> i32 {
  ///     k.abs()
  ///   }
  ///   v.const_sort_unstable_by_key(pred);
  ///   v
  /// };
  /// assert_eq!(V, [1, 2, -3, 4, -5]);
  /// ```
  ///
  /// [pdqsort]: https://github.com/orlp/pdqsort
  fn const_sort_unstable_by_key<K, F>(&mut self, f: F)
  where
    F: FnMut(&T) -> K,
    K: Ord;

  /// Reorder the slice such that the element at `index` is at its final sorted position.
  ///
  /// This reordering has the additional property that any value at position `i < index` will be
  /// less than or equal to any value at a position `j > index`. Additionally, this reordering is
  /// unstable (i.e. any number of equal elements may end up at position `index`), in-place
  /// (i.e. does not allocate), and *O*(*n*) worst-case. This function is also/ known as "kth
  /// element" in other libraries. It returns a triplet of the following values: all elements less
  /// than the one at the given index, the value at the given index, and all elements greater than
  /// the one at the given index.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on the quickselect portion of the same quicksort algorithm
  /// used for [`sort_unstable`].
  ///
  /// [`sort_unstable`]: slice::sort_unstable
  ///
  /// # Panics
  ///
  /// Panics when `index >= len()`, meaning it always panics on empty slices.
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// use const_sort_rs::ConstSliceSortExt;
  /// const V: [i32; 5] = {
  ///   let mut v = [-5i32, 4, 1, -3, 2];
  ///   // Find the median
  ///   v.const_select_nth_unstable(2);
  ///   v
  /// };
  /// // We are only guaranteed the slice will be one of the following, based on the way we sort
  /// // about the specified index.
  /// assert!(
  ///   V == [-3, -5, 1, 2, 4]
  ///     || V == [-5, -3, 1, 2, 4]
  ///     || V == [-3, -5, 1, 4, 2]
  ///     || V == [-5, -3, 1, 4, 2]
  /// );
  /// ```
  fn const_select_nth_unstable(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
  where
    T: Ord;
  /// Reorder the slice with a comparator function such that the element at `index` is at its
  /// final sorted position.
  ///
  /// This reordering has the additional property that any value at position `i < index` will be
  /// less than or equal to any value at a position `j > index` using the comparator function.
  /// Additionally, this reordering is unstable (i.e. any number of equal elements may end up at
  /// position `index`), in-place (i.e. does not allocate), and *O*(*n*) worst-case. This function
  /// is also known as "kth element" in other libraries. It returns a triplet of the following
  /// values: all elements less than the one at the given index, the value at the given index,
  /// and all elements greater than the one at the given index, using the provided comparator
  /// function.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on the quickselect portion of the same quicksort algorithm
  /// used for [`sort_unstable`].
  ///
  /// [`sort_unstable`]: slice::sort_unstable
  ///
  /// # Panics
  ///
  /// Panics when `index >= len()`, meaning it always panics on empty slices.
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// #![feature(const_cmp)]
  /// # use core::cmp::Ordering;
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const V: [i32; 5] = {
  ///   let mut v = [-5i32, 4, 1, -3, 2];
  ///   // no const closures yet
  ///   const fn pred(a: &i32, b: &i32) -> Ordering {
  ///     b.cmp(a)
  ///   }
  ///   // Find the median as if the slice were sorted in descending order.
  ///   v.const_select_nth_unstable_by(2, pred);
  ///   v
  /// };
  ///
  /// // We are only guaranteed the slice will be one of the following, based on the way we sort
  /// // about the specified index.
  /// assert!(
  ///   V == [2, 4, 1, -5, -3]
  ///     || V == [2, 4, 1, -3, -5]
  ///     || V == [4, 2, 1, -5, -3]
  ///     || V == [4, 2, 1, -3, -5]
  /// );
  /// ```
  fn const_select_nth_unstable_by<F>(
    &mut self,
    index: usize,
    compare: F,
  ) -> (&mut [T], &mut T, &mut [T])
  where
    F: FnMut(&T, &T) -> Ordering;
  /// Reorder the slice with a key extraction function such that the element at `index` is at its
  /// final sorted position.
  ///
  /// This reordering has the additional property that any value at position `i < index` will be
  /// less than or equal to any value at a position `j > index` using the key extraction function.
  /// Additionally, this reordering is unstable (i.e. any number of equal elements may end up at
  /// position `index`), in-place (i.e. does not allocate), and *O*(*n*) worst-case. This function
  /// is also known as "kth element" in other libraries. It returns a triplet of the following
  /// values: all elements less than the one at the given index, the value at the given index, and
  /// all elements greater than the one at the given index, using the provided key extraction
  /// function.
  ///
  /// # Current implementation
  ///
  /// The current algorithm is based on the quickselect portion of the same quicksort algorithm
  /// used for [`sort_unstable`].
  ///
  /// [`sort_unstable`]: slice::sort_unstable
  ///
  /// # Panics
  ///
  /// Panics when `index >= len()`, meaning it always panics on empty slices.
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// #![feature(const_cmp)]
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const V: [i32; 5] = {
  ///   let mut v = [-5i32, 4, 1, -3, 2];
  ///   // no const closures yet
  ///   const fn pred(k: &i32) -> i32 {
  ///     k.abs()
  ///   }
  ///   // Return the median as if the array were sorted according to absolute value.
  ///   v.const_select_nth_unstable_by_key(2, pred);
  ///   v
  /// };
  ///
  /// // We are only guaranteed the slice will be one of the following, based on the way we sort
  /// // about the specified index.
  /// assert!(
  ///   V == [1, 2, -3, 4, -5]
  ///     || V == [1, 2, -3, -5, 4]
  ///     || V == [2, 1, -3, 4, -5]
  ///     || V == [2, 1, -3, -5, 4]
  /// );
  /// ```
  fn const_select_nth_unstable_by_key<K, F>(
    &mut self,
    index: usize,
    f: F,
  ) -> (&mut [T], &mut T, &mut [T])
  where
    F: FnMut(&T) -> K,
    K: Ord;

  /// Checks if the elements of this slice are sorted.
  ///
  /// That is, for each element `a` and its following element `b`, `a <= b` must hold. If the
  /// slice yields exactly zero or one element, `true` is returned.
  ///
  /// Note that if `Self::Item` is only `PartialOrd`, but not `Ord`, the above definition
  /// implies that this function returns `false` if any two consecutive items are not
  /// comparable.
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const A: bool = [1, 2, 2, 9].const_is_sorted();
  /// assert!(A);
  /// const B: bool = [1, 3, 2, 4].const_is_sorted();
  /// assert!(!B);
  /// const C: bool = [0].const_is_sorted();
  /// assert!(C);
  /// const EMPTY: [i32; 0] = [];
  /// const D: bool = EMPTY.const_is_sorted();
  /// assert!(D);
  /// const E: bool = [0.0, 1.0, f32::NAN].const_is_sorted();
  /// assert!(!E);
  /// ```
  #[must_use]
  fn const_is_sorted(&self) -> bool
  where
    T: PartialOrd;
  /// Checks if the elements of this slice are sorted using the given comparator function.
  ///
  /// Instead of using `PartialOrd::partial_cmp`, this function uses the given `compare`
  /// function to determine the ordering of two elements. Apart from that, it's equivalent to
  /// [`is_sorted`]; see its documentation for more information.
  ///
  /// [`is_sorted`]: slice::is_sorted
  #[must_use]
  fn const_is_sorted_by<F>(&self, compare: F) -> bool
  where
    F: FnMut(&T, &T) -> Option<Ordering>;
  /// Checks if the elements of this slice are sorted using the given key extraction function.
  ///
  /// Instead of comparing the slice's elements directly, this function compares the keys of the
  /// elements, as determined by `f`. Apart from that, it's equivalent to [`is_sorted`]; see its
  /// documentation for more information.
  ///
  /// [`is_sorted`]: slice::is_sorted
  ///
  /// # Examples
  ///
  /// ```
  /// #![feature(const_mut_refs)]
  /// #![feature(const_trait_impl)]
  /// use const_sort_rs::ConstSliceSortExt;
  ///
  /// const fn map_len(s: &&str) -> usize {
  ///   s.len()
  /// }
  /// const A: bool = ["c", "bb", "aaa"].const_is_sorted_by_key(map_len);
  /// assert!(A);
  ///
  /// const fn map_abs(i: &i32) -> i32 {
  ///   i.abs()
  /// }
  /// const B: bool = [-2i32, -1, 0, 3].const_is_sorted_by_key(map_abs);
  /// assert!(!B);
  /// ```
  #[must_use]
  fn const_is_sorted_by_key<F, K>(&self, f: F) -> bool
  where
    F: FnMut(&T) -> K,
    K: PartialOrd;
}

impl<T> const ConstSliceSortExt<T> for [T] {
  #[inline]
  fn const_sort_unstable(&mut self)
  where
    T: ~const PartialOrd + Ord,
  {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2539
    const_sort::const_quicksort(self, PartialOrd::lt);
  }
  #[inline]
  fn const_sort_unstable_by<F>(&mut self, mut compare: F)
  where
    F: ~const FnMut(&T, &T) -> Ordering + ~const Destruct,
  {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2594
    const_sort::const_quicksort(self, const |a, b| compare(a, b) == Ordering::Less);
  }
  #[inline]
  fn const_sort_unstable_by_key<K, F>(&mut self, mut f: F)
  where
    F: ~const FnMut(&T) -> K + ~const Destruct,
    K: Ord + ~const PartialOrd + ~const Destruct,
  {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2632
    const_sort::const_quicksort(self, const |a, b| f(a).lt(&f(b)));
  }

  #[inline]
  fn const_select_nth_unstable(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
  where
    T: ~const PartialOrd + Ord,
  {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2678
    const_sort::const_partition_at_index(self, index, PartialOrd::lt)
  }
  #[inline]
  fn const_select_nth_unstable_by<F>(
    &mut self,
    index: usize,
    mut compare: F,
  ) -> (&mut [T], &mut T, &mut [T])
  where
    F: ~const FnMut(&T, &T) -> Ordering + ~const Destruct,
  {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2725
    let mut f = const |a: &T, b: &T| compare(a, b) == Ordering::Less;
    const_sort::const_partition_at_index(self, index, &mut f)
  }
  #[inline]
  fn const_select_nth_unstable_by_key<K, F>(
    &mut self,
    index: usize,
    mut f: F,
  ) -> (&mut [T], &mut T, &mut [T])
  where
    F: ~const FnMut(&T) -> K + ~const Destruct,
    K: Ord + ~const PartialOrd + ~const Destruct,
  {
    // https://doc.rust-lang.org/nightly/src/core/slice/mod.rs.html#2776
    let mut g = const |a: &T, b: &T| f(a).lt(&f(b));
    const_sort::const_partition_at_index(self, index, &mut g)
  }

  #[inline]
  fn const_is_sorted(&self) -> bool
  where
    T: ~const PartialOrd,
  {
    self.const_is_sorted_by(PartialOrd::partial_cmp)
  }
  fn const_is_sorted_by<F>(&self, mut compare: F) -> bool
  where
    F: ~const FnMut(&T, &T) -> Option<Ordering> + ~const Destruct,
  {
    // https://doc.rust-lang.org/nightly/src/core/iter/traits/iterator.rs.html#3794
    let mut i = 1;
    while i < self.len() {
      let ord_opt = compare(&self[i - 1], &self[i]);
      if ord_opt.is_none() {
        return false;
      }
      if ord_opt.unwrap() == Ordering::Greater {
        return false;
      }
      i += 1;
    }
    true
  }
  #[inline]
  fn const_is_sorted_by_key<F, K>(&self, mut f: F) -> bool
  where
    F: ~const FnMut(&T) -> K + ~const Destruct,
    K: ~const PartialOrd + ~const Destruct,
  {
    self.const_is_sorted_by(const |a, b| f(a).partial_cmp(&f(b)))
  }
}
