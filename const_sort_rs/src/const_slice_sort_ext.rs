use core::{
  cmp::Ordering,
  marker::{Destruct, PhantomData},
};

use crate::const_sort;

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
  ///   let mut x = [-5, 4, 1, -3, 2];
  ///   x.const_sort_unstable();
  ///   x
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
}

pub(crate) const fn const_pred_lt<T: Ord + ~const PartialOrd>(a: &T, b: &T) -> bool {
  a.lt(b)
}

impl<T: ~const PartialOrd> const ConstSliceSortExt<T> for [T] {
  #[inline]
  fn const_sort_unstable(&mut self)
  where
    T: Ord,
  {
    const_sort::const_quicksort(self, const_pred_lt);
  }
  #[inline]
  fn const_sort_unstable_by<F>(&mut self, compare: F)
  where
    F: ~const FnMut(&T, &T) -> Ordering + ~const Destruct,
  {
    // sort::const_quicksort(self, |a, b| compare(a, b) == Ordering::Less);
    struct ClosureHelperBy<F, T>
    where
      F: FnMut(&T, &T) -> Ordering,
    {
      compare_fn: F,
      _t: PhantomData<*const T>,
    }

    impl<F, T> const FnOnce<(&T, &T)> for ClosureHelperBy<F, T>
    where
      F: ~const FnMut(&T, &T) -> Ordering,
      Self: ~const Destruct,
    {
      type Output = bool;

      extern "rust-call" fn call_once(mut self, args: (&T, &T)) -> Self::Output {
        self.call_mut(args)
      }
    }

    impl<F, T> const FnMut<(&T, &T)> for ClosureHelperBy<F, T>
    where
      F: ~const FnMut(&T, &T) -> Ordering,
    {
      extern "rust-call" fn call_mut(&mut self, (a, b): (&T, &T)) -> Self::Output {
        matches!((self.compare_fn)(a, b), Ordering::Less)
      }
    }
    const_sort::const_quicksort(
      self,
      ClosureHelperBy {
        compare_fn: compare,
        _t: PhantomData,
      },
    );
  }
  #[inline]
  fn const_sort_unstable_by_key<K, F>(&mut self, f: F)
  where
    F: ~const FnMut(&T) -> K + ~const Destruct,
    K: Ord + ~const PartialOrd + ~const Destruct,
  {
    struct ClosureHelperByKey<F, T, K: Ord>
    where
      F: ~const FnMut(&T) -> K,
    {
      by_key_fn: F,
      _t: PhantomData<*const T>,
      _k: PhantomData<*const K>,
    }

    impl<F, T, K: Ord + ~const PartialOrd + ~const Destruct> const FnOnce<(&T, &T)>
      for ClosureHelperByKey<F, T, K>
    where
      F: ~const FnMut(&T) -> K,
      Self: ~const Destruct,
    {
      type Output = bool;

      extern "rust-call" fn call_once(mut self, args: (&T, &T)) -> Self::Output {
        self.call_mut(args)
      }
    }

    impl<F, T, K: Ord + ~const PartialOrd + ~const Destruct> const FnMut<(&T, &T)>
      for ClosureHelperByKey<F, T, K>
    where
      F: ~const FnMut(&T) -> K,
    {
      extern "rust-call" fn call_mut(&mut self, (a, b): (&T, &T)) -> Self::Output {
        (self.by_key_fn)(a).lt(&(self.by_key_fn)(b))
      }
    }
    const_sort::const_quicksort(
      self,
      ClosureHelperByKey {
        by_key_fn: f,
        _k: PhantomData,
        _t: PhantomData,
      },
    );
  }
}
