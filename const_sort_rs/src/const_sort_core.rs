//! Slice sorting
//!
//! This module contains a sorting algorithm based on Orson Peters' pattern-defeating quicksort,
//! published at: <https://github.com/orlp/pdqsort>
//!
//! Unstable sorting is compatible with libcore because it doesn't allocate memory, unlike our
//! stable sorting implementation.

// https://doc.rust-lang.org/src/core/slice/sort.rs.html

use core::cmp;
use core::marker::Destruct;
use core::mem::{self, MaybeUninit};
use core::ptr;

use crate::utils::split_at_mut_unchecked;

pub trait ConstUnstableSortable {
  fn const_heapsort(&mut self);
  fn const_quicksort(&mut self);
}

const fn const_pred_lt<T: Ord + ~const PartialOrd>(a: &T, b: &T) -> bool {
  a.lt(b)
}

impl<T: Ord + ~const PartialOrd> const ConstUnstableSortable for [T] {
  fn const_heapsort(&mut self) {
    const_heapsort(self, &mut const_pred_lt)
  }

  fn const_quicksort(&mut self) {
    const_quicksort(self, const_pred_lt)
  }
}

/// When dropped, copies from `src` into `dest`.
struct CopyOnDrop<T> {
  src: *const T,
  dest: *mut T,
}

impl<T> const Drop for CopyOnDrop<T> {
  fn drop(&mut self) {
    // SAFETY:  This is a helper class.
    //          Please refer to its usage for correctness.
    //          Namely, one must be sure that `src` and `dst` does not overlap as required by `ptr::copy_nonoverlapping`.
    unsafe {
      ptr::copy_nonoverlapping(self.src, self.dest, 1);
    }
  }
}

/// Shifts the first element to the right until it encounters a greater or equal element.
const fn shift_head<T, F>(v: &mut [T], is_less: &mut F)
where
  F: ~const FnMut(&T, &T) -> bool,
{
  let len = v.len();
  // SAFETY: The unsafe operations below involves indexing without a bounds check (by offsetting a
  // pointer) and copying memory (`ptr::copy_nonoverlapping`).
  //
  // a. Indexing:
  //  1. We checked the size of the array to >=2.
  //  2. All the indexing that we will do is always between {0 <= index < len} at most.
  //
  // b. Memory copying
  //  1. We are obtaining pointers to references which are guaranteed to be valid.
  //  2. They cannot overlap because we obtain pointers to difference indices of the slice.
  //     Namely, `i` and `i-1`.
  //  3. If the slice is properly aligned, the elements are properly aligned.
  //     It is the caller's responsibility to make sure the slice is properly aligned.
  //
  // See comments below for further detail.
  unsafe {
    // If the first two elements are out-of-order...
    if len >= 2 && is_less(v.get_unchecked(1), v.get_unchecked(0)) {
      // Read the first element into a stack-allocated variable. If a following comparison
      // operation panics, `hole` will get dropped and automatically write the element back
      // into the slice.
      let tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(0)));
      let v = v.as_mut_ptr();
      let mut hole = CopyOnDrop {
        src: &*tmp,
        dest: v.add(1),
      };
      ptr::copy_nonoverlapping(v.add(1), v.add(0), 1);

      // for i in 2..len {
      let mut i = 2;
      while i < len {
        if !is_less(&*v.add(i), &*tmp) {
          break;
        }

        // Move `i`-th element one place to the left, thus shifting the hole to the right.
        ptr::copy_nonoverlapping(v.add(i), v.add(i - 1), 1);
        hole.dest = v.add(i);
        i += 1;
      }
      // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
    }
  }
}

/// Shifts the last element to the left until it encounters a smaller or equal element.
const fn shift_tail<T, F>(v: &mut [T], is_less: &mut F)
where
  F: ~const FnMut(&T, &T) -> bool,
{
  let len = v.len();
  // SAFETY: The unsafe operations below involves indexing without a bound check (by offsetting a
  // pointer) and copying memory (`ptr::copy_nonoverlapping`).
  //
  // a. Indexing:
  //  1. We checked the size of the array to >= 2.
  //  2. All the indexing that we will do is always between `0 <= index < len-1` at most.
  //
  // b. Memory copying
  //  1. We are obtaining pointers to references which are guaranteed to be valid.
  //  2. They cannot overlap because we obtain pointers to difference indices of the slice.
  //     Namely, `i` and `i+1`.
  //  3. If the slice is properly aligned, the elements are properly aligned.
  //     It is the caller's responsibility to make sure the slice is properly aligned.
  //
  // See comments below for further detail.
  unsafe {
    // If the last two elements are out-of-order...
    if len >= 2 && is_less(v.get_unchecked(len - 1), v.get_unchecked(len - 2)) {
      // Read the last element into a stack-allocated variable. If a following comparison
      // operation panics, `hole` will get dropped and automatically write the element back
      // into the slice.
      let tmp = mem::ManuallyDrop::new(ptr::read(v.get_unchecked(len - 1)));
      let v = v.as_mut_ptr();
      let mut hole = CopyOnDrop {
        src: &*tmp,
        dest: v.add(len - 2),
      };
      ptr::copy_nonoverlapping(v.add(len - 2), v.add(len - 1), 1);

      // for i in (0..len - 2).rev() {
      let mut i = len - 2;
      while i > 0 {
        i -= 1;
        if !is_less(&*tmp, &*v.add(i)) {
          break;
        }

        // Move `i`-th element one place to the right, thus shifting the hole to the left.
        ptr::copy_nonoverlapping(v.add(i), v.add(i + 1), 1);
        hole.dest = v.add(i);
      }
      // `hole` gets dropped and thus copies `tmp` into the remaining hole in `v`.
    }
  }
}

/// Partially sorts a slice by shifting several out-of-order elements around.
///
/// Returns `true` if the slice is sorted at the end. This function is *O*(*n*) worst-case.
#[cold]
const fn partial_insertion_sort<T, F>(v: &mut [T], is_less: &mut F) -> bool
where
  F: ~const FnMut(&T, &T) -> bool,
{
  // Maximum number of adjacent out-of-order pairs that will get shifted.
  const MAX_STEPS: usize = 5;
  // If the slice is shorter than this, don't shift any elements.
  const SHORTEST_SHIFTING: usize = 50;

  let len = v.len();
  let mut i = 1;

  // for _ in 0..MAX_STEPS {
  let mut oi = 0;
  while oi < MAX_STEPS {
    // SAFETY: We already explicitly did the bound checking with `i < len`.
    // All our subsequent indexing is only in the range `0 <= index < len`
    unsafe {
      // Find the next pair of adjacent out-of-order elements.
      while i < len && !is_less(v.get_unchecked(i), v.get_unchecked(i - 1)) {
        i += 1;
      }
    }

    // Are we done?
    if i == len {
      return true;
    }

    // Don't shift elements on short arrays, that has a performance cost.
    if len < SHORTEST_SHIFTING {
      return false;
    }

    // Swap the found pair of elements. This puts them in correct order.
    v.swap(i - 1, i);

    // Shift the smaller element to the left.
    shift_tail(&mut v[..i], is_less);
    // Shift the greater element to the right.
    shift_head(&mut v[i..], is_less);

    oi += 1;
  }

  // Didn't manage to sort the slice in the limited number of steps.
  false
}

/// Sorts a slice using insertion sort, which is *O*(*n*^2) worst-case.
const fn insertion_sort<T, F>(v: &mut [T], is_less: &mut F)
where
  F: ~const FnMut(&T, &T) -> bool,
{
  // for i in 1..v.len() {
  let mut i = 1;
  while i < v.len() {
    shift_tail(&mut v[..i + 1], is_less);
    i += 1;
  }
}

/// This binary heap respects the invariant `parent >= child`.
const fn sift_down<T, F>(v: &mut [T], mut node: usize, is_less: &mut F)
where
  F: ~const FnMut(&T, &T) -> bool,
{
  loop {
    // Children of `node`.
    let mut child = 2 * node + 1;
    if child >= v.len() {
      break;
    }

    // Choose the greater child.
    if child + 1 < v.len() && is_less(&v[child], &v[child + 1]) {
      child += 1;
    }

    // Stop if the invariant holds at `node`.
    if !is_less(&v[node], &v[child]) {
      break;
    }

    // Swap `node` with the greater child, move one step down, and continue sifting.
    v.swap(node, child);
    node = child;
  }
}

/// Sorts `v` using heapsort, which guarantees *O*(*n* \* log(*n*)) worst-case.
#[cold]
pub const fn const_heapsort<T, F>(v: &mut [T], is_less: &mut F)
where
  F: ~const FnMut(&T, &T) -> bool + ~const Destruct,
{
  // Build the heap in linear time.
  //for i in (0..v.len() / 2).rev() {
  let mut i = isize::try_from(v.len() / 2 - 1).ok().unwrap();
  while i >= 0 {
    sift_down(v, i as usize, is_less);
    i -= 1;
  }

  // Pop maximal elements from the heap.
  // for i in (1..v.len()).rev() {
  let mut i = v.len() - 1;
  while i >= 1 {
    v.swap(0, i);
    sift_down(&mut v[..i], 0, is_less);
    i -= 1;
  }
}

/// Partitions `v` into elements smaller than `pivot`, followed by elements greater than or equal
/// to `pivot`.
///
/// Returns the number of elements smaller than `pivot`.
///
/// Partitioning is performed block-by-block in order to minimize the cost of branching operations.
/// This idea is presented in the [BlockQuicksort][pdf] paper.
///
/// [pdf]: https://drops.dagstuhl.de/opus/volltexte/2016/6389/pdf/LIPIcs-ESA-2016-38.pdf
const fn partition_in_blocks<T, F>(v: &mut [T], pivot: &T, is_less: &mut F) -> usize
where
  F: ~const FnMut(&T, &T) -> bool,
{
  // Number of elements in a typical block.
  const BLOCK: usize = 128;

  // The partitioning algorithm repeats the following steps until completion:
  //
  // 1. Trace a block from the left side to identify elements greater than or equal to the pivot.
  // 2. Trace a block from the right side to identify elements smaller than the pivot.
  // 3. Exchange the identified elements between the left and right side.
  //
  // We keep the following variables for a block of elements:
  //
  // 1. `block` - Number of elements in the block.
  // 2. `start` - Start pointer into the `offsets` array.
  // 3. `end` - End pointer into the `offsets` array.
  // 4. `offsets - Indices of out-of-order elements within the block.

  // The current block on the left side (from `l` to `l.add(block_l)`).
  let mut l = v.as_mut_ptr();
  let mut block_l = BLOCK;
  let mut start_l = 0; // indexes offsets_l
  let mut end_l = 0; // holds end of offsets_l
  let mut offsets_l = [MaybeUninit::<u8>::uninit(); BLOCK];

  // The current block on the right side (from `r.sub(block_r)` to `r`).
  // SAFETY: The documentation for .add() specifically mention that `vec.as_ptr().add(vec.len())` is always safe`
  let mut r = unsafe { l.add(v.len()) };
  let mut block_r = BLOCK;
  let mut start_r = 0; // indexes offsets_r
  let mut end_r = 0; // holds end of offsets_r
  let mut offsets_r = [MaybeUninit::<u8>::uninit(); BLOCK];

  // FIXME: When we get VLAs, try creating one array of length `min(v.len(), 2 * BLOCK)` rather
  // than two fixed-size arrays of length `BLOCK`. VLAs might be more cache-efficient.

  // Returns the number of elements between pointers `l` (inclusive) and `r` (exclusive).
  const fn width<T>(l: *mut T, r: *mut T) -> usize {
    assert!(mem::size_of::<T>() > 0);
    // FIXME: this should *likely* use `offset_from`, but more
    // investigation is needed (including running tests in miri).
    (unsafe { (mem::transmute::<_, usize>(r)) - (mem::transmute::<_, usize>(l)) })
      / mem::size_of::<T>()
  }

  // Returns the number of elements between pointers `l` (inclusive) and `r` (exclusive).
  const fn width2(l: usize, r: usize) -> usize {
    r - l
  }

  loop {
    // We are done with partitioning block-by-block when `l` and `r` get very close. Then we do
    // some patch-up work in order to partition the remaining elements in between.
    let is_done = width(l, r) <= 2 * BLOCK;

    if is_done {
      // Number of remaining elements (still not compared to the pivot).
      let mut rem = width(l, r);
      if start_l < end_l || start_r < end_r {
        rem -= BLOCK;
      }

      // Adjust block sizes so that the left and right block don't overlap, but get perfectly
      // aligned to cover the whole remaining gap.
      if start_l < end_l {
        block_r = rem;
      } else if start_r < end_r {
        block_l = rem;
      } else {
        // There were the same number of elements to switch on both blocks during the last
        // iteration, so there are no remaining elements on either block. Cover the remaining
        // items with roughly equally-sized blocks.
        block_l = rem / 2;
        block_r = rem - block_l;
      }
      debug_assert!(block_l <= BLOCK && block_r <= BLOCK);
      debug_assert!(width(l, r) == block_l + block_r);
    }

    if start_l == end_l {
      // Trace `block_l` elements from the left side.
      start_l = 0;
      end_l = start_l;
      let mut elem = l;

      // for i in 0..block_l {
      let mut i = 0;
      while i < block_l {
        // SAFETY: The unsafety operations below involve the usage of the `offset`.
        //         According to the conditions required by the function, we satisfy them because:
        //         1. `offsets_l` is stack-allocated, and thus considered separate allocated object.
        //         2. The function `is_less` returns a `bool`.
        //            Casting a `bool` will never overflow `isize`.
        //         3. We have guaranteed that `block_l` will be `<= BLOCK`.
        //            Plus, `end_l` was initially set to the begin pointer of `offsets_` which was declared on the stack.
        //            Thus, we know that even in the worst case (all invocations of `is_less` returns false) we will only be at most 1 byte pass the end.
        //        Another unsafety operation here is dereferencing `elem`.
        //        However, `elem` was initially the begin pointer to the slice which is always valid.
        unsafe {
          // Branchless comparison.
          offsets_l[end_l].write(i as u8);
          end_l = end_l
            .checked_add_signed(!is_less(&*elem, pivot) as isize)
            .unwrap();
          elem = elem.offset(1);
        }
        i += 1;
      }
    }

    if start_r == end_r {
      // Trace `block_r` elements from the right side.
      start_r = 0;
      end_r = start_r;
      let mut elem = r;

      // for i in 0..block_r {
      let mut i = 0;
      while i < block_r {
        // SAFETY: The unsafety operations below involve the usage of the `offset`.
        //         According to the conditions required by the function, we satisfy them because:
        //         1. `offsets_r` is stack-allocated, and thus considered separate allocated object.
        //         2. The function `is_less` returns a `bool`.
        //            Casting a `bool` will never overflow `isize`.
        //         3. We have guaranteed that `block_r` will be `<= BLOCK`.
        //            Plus, `end_r` was initially set to the begin pointer of `offsets_` which was declared on the stack.
        //            Thus, we know that even in the worst case (all invocations of `is_less` returns true) we will only be at most 1 byte pass the end.
        //        Another unsafety operation here is dereferencing `elem`.
        //        However, `elem` was initially `1 * sizeof(T)` past the end and we decrement it by `1 * sizeof(T)` before accessing it.
        //        Plus, `block_r` was asserted to be less than `BLOCK` and `elem` will therefore at most be pointing to the beginning of the slice.
        unsafe {
          // Branchless comparison.
          elem = elem.offset(-1);
          offsets_r[end_r].write(i as u8);
          end_r = end_r
            .checked_add_signed(is_less(&*elem, pivot) as isize)
            .unwrap();
        }
        i += 1;
      }
    }

    // Number of out-of-order elements to swap between the left and right side.
    let count = cmp::min(width2(start_l, end_l), width2(start_r, end_r));

    if count > 0 {
      macro_rules! left {
        () => {
          l.offset(offsets_l[start_l].assume_init() as isize)
        };
      }
      macro_rules! right {
        () => {
          r.offset(-(offsets_r[start_r].assume_init() as isize) - 1)
        };
      }

      // Instead of swapping one pair at the time, it is more efficient to perform a cyclic
      // permutation. This is not strictly equivalent to swapping, but produces a similar
      // result using fewer memory operations.

      // SAFETY: The use of `ptr::read` is valid because there is at least one element in
      // both `offsets_l` and `offsets_r`, so `left!` is a valid pointer to read from.
      //
      // The uses of `left!` involve calls to `offset` on `l`, which points to the
      // beginning of `v`. All the offsets pointed-to by `start_l` are at most `block_l`, so
      // these `offset` calls are safe as all reads are within the block. The same argument
      // applies for the uses of `right!`.
      //
      // The calls to `start_l.offset` are valid because there are at most `count-1` of them,
      // plus the final one at the end of the unsafe block, where `count` is the minimum number
      // of collected offsets in `offsets_l` and `offsets_r`, so there is no risk of there not
      // being enough elements. The same reasoning applies to the calls to `start_r.offset`.
      //
      // The calls to `copy_nonoverlapping` are safe because `left!` and `right!` are guaranteed
      // not to overlap, and are valid because of the reasoning above.
      unsafe {
        let tmp = ptr::read(left!());
        ptr::copy_nonoverlapping(right!(), left!(), 1);

        // for _ in 1..count {
        let mut oi = 1;
        while oi < count {
          start_l = start_l + 1;
          ptr::copy_nonoverlapping(left!(), right!(), 1);
          start_r = start_r + 1;
          ptr::copy_nonoverlapping(right!(), left!(), 1);
          oi += 1;
        }

        ptr::copy_nonoverlapping(&tmp, right!(), 1);
        mem::forget(tmp);
        start_l = start_l + 1;
        start_r = start_r + 1;
      }
    }

    if start_l == end_l {
      // All out-of-order elements in the left block were moved. Move to the next block.

      // block-width-guarantee
      // SAFETY: if `!is_done` then the slice width is guaranteed to be at least `2*BLOCK` wide. There
      // are at most `BLOCK` elements in `offsets_l` because of its size, so the `offset` operation is
      // safe. Otherwise, the debug assertions in the `is_done` case guarantee that
      // `width(l, r) == block_l + block_r`, namely, that the block sizes have been adjusted to account
      // for the smaller number of remaining elements.
      l = unsafe { l.offset(block_l as isize) };
    }

    if start_r == end_r {
      // All out-of-order elements in the right block were moved. Move to the previous block.

      // SAFETY: Same argument as [block-width-guarantee]. Either this is a full block `2*BLOCK`-wide,
      // or `block_r` has been adjusted for the last handful of elements.
      r = unsafe { r.offset(-(block_r as isize)) };
    }

    if is_done {
      break;
    }
  }

  // All that remains now is at most one block (either the left or the right) with out-of-order
  // elements that need to be moved. Such remaining elements can be simply shifted to the end
  // within their block.

  if start_l < end_l {
    // The left block remains.
    // Move its remaining out-of-order elements to the far right.
    // debug_assert_eq!(width(l, r), block_l);
    while start_l < end_l {
      // remaining-elements-safety
      // SAFETY: while the loop condition holds there are still elements in `offsets_l`, so it
      // is safe to point `end_l` to the previous element.
      //
      // The `ptr::swap` is safe if both its arguments are valid for reads and writes:
      //  - Per the debug assert above, the distance between `l` and `r` is `block_l`
      //    elements, so there can be at most `block_l` remaining offsets between `start_l`
      //    and `end_l`. This means `r` will be moved at most `block_l` steps back, which
      //    makes the `r.offset` calls valid (at that point `l == r`).
      //  - `offsets_l` contains valid offsets into `v` collected during the partitioning of
      //    the last block, so the `l.offset` calls are valid.
      unsafe {
        end_l = end_l - 1;
        ptr::swap(
          l.offset(offsets_l[end_l].assume_init() as isize),
          r.offset(-1),
        );
        r = r.offset(-1);
      }
    }
    width(v.as_mut_ptr(), r)
  } else if start_r < end_r {
    // The right block remains.
    // Move its remaining out-of-order elements to the far left.
    // debug_assert_eq!(width(l, r), block_r);
    while start_r < end_r {
      // SAFETY: See the reasoning in [remaining-elements-safety].
      unsafe {
        end_r = end_r - 1;
        ptr::swap(l, r.offset(-(offsets_r[end_r].assume_init() as isize) - 1));
        l = l.offset(1);
      }
    }
    width(v.as_mut_ptr(), l)
  } else {
    // Nothing else to do, we're done.
    width(v.as_mut_ptr(), l)
  }
}

/// Partitions `v` into elements smaller than `v[pivot]`, followed by elements greater than or
/// equal to `v[pivot]`.
///
/// Returns a tuple of:
///
/// 1. Number of elements smaller than `v[pivot]`.
/// 2. True if `v` was already partitioned.
const fn partition<T, F>(v: &mut [T], pivot: usize, is_less: &mut F) -> (usize, bool)
where
  F: ~const FnMut(&T, &T) -> bool,
{
  let (mid, was_partitioned) = {
    // Place the pivot at the beginning of slice.
    v.swap(0, pivot);
    let (pivot, v) = unsafe { split_at_mut_unchecked(v, 1) };
    let pivot = &mut pivot[0];

    // Read the pivot into a stack-allocated variable for efficiency. If a following comparison
    // operation panics, the pivot will be automatically written back into the slice.

    // SAFETY: `pivot` is a reference to the first element of `v`, so `ptr::read` is safe.
    let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
    let _pivot_guard = CopyOnDrop {
      src: &*tmp,
      dest: pivot,
    };
    let pivot = &*tmp;

    // Find the first pair of out-of-order elements.
    let mut l = 0;
    let mut r = v.len();

    // SAFETY: The unsafety below involves indexing an array.
    // For the first one: We already do the bounds checking here with `l < r`.
    // For the second one: We initially have `l == 0` and `r == v.len()` and we checked that `l < r` at every indexing operation.
    //                     From here we know that `r` must be at least `r == l` which was shown to be valid from the first one.
    unsafe {
      // Find the first element greater than or equal to the pivot.
      while l < r && is_less(v.get_unchecked(l), pivot) {
        l += 1;
      }

      // Find the last element smaller that the pivot.
      while l < r && !is_less(v.get_unchecked(r - 1), pivot) {
        r -= 1;
      }
    }

    (
      l + partition_in_blocks(&mut v[l..r], pivot, is_less),
      l >= r,
    )

    // `_pivot_guard` goes out of scope and writes the pivot (which is a stack-allocated
    // variable) back into the slice where it originally was. This step is critical in ensuring
    // safety!
  };

  // Place the pivot between the two partitions.
  v.swap(0, mid);

  (mid, was_partitioned)
}

/// Partitions `v` into elements equal to `v[pivot]` followed by elements greater than `v[pivot]`.
///
/// Returns the number of elements equal to the pivot. It is assumed that `v` does not contain
/// elements smaller than the pivot.
const fn partition_equal<T, F>(v: &mut [T], pivot: usize, is_less: &mut F) -> usize
where
  F: ~const FnMut(&T, &T) -> bool,
{
  // Place the pivot at the beginning of slice.
  v.swap(0, pivot);
  let (pivot, v) = unsafe { split_at_mut_unchecked(v, 1) };
  let pivot = &mut pivot[0];

  // Read the pivot into a stack-allocated variable for efficiency. If a following comparison
  // operation panics, the pivot will be automatically written back into the slice.
  // SAFETY: The pointer here is valid because it is obtained from a reference to a slice.
  let tmp = mem::ManuallyDrop::new(unsafe { ptr::read(pivot) });
  let _pivot_guard = CopyOnDrop {
    src: &*tmp,
    dest: pivot,
  };
  let pivot = &*tmp;

  // Now partition the slice.
  let mut l = 0;
  let mut r = v.len();
  loop {
    // SAFETY: The unsafety below involves indexing an array.
    // For the first one: We already do the bounds checking here with `l < r`.
    // For the second one: We initially have `l == 0` and `r == v.len()` and we checked that `l < r` at every indexing operation.
    //                     From here we know that `r` must be at least `r == l` which was shown to be valid from the first one.
    unsafe {
      // Find the first element greater than the pivot.
      while l < r && !is_less(pivot, v.get_unchecked(l)) {
        l += 1;
      }

      // Find the last element equal to the pivot.
      while l < r && is_less(pivot, v.get_unchecked(r - 1)) {
        r -= 1;
      }

      // Are we done?
      if l >= r {
        break;
      }

      // Swap the found pair of out-of-order elements.
      r -= 1;
      let ptr = v.as_mut_ptr();
      ptr::swap(ptr.add(l), ptr.add(r));
      l += 1;
    }
  }

  // We found `l` elements equal to the pivot. Add 1 to account for the pivot itself.
  l + 1

  // `_pivot_guard` goes out of scope and writes the pivot (which is a stack-allocated variable)
  // back into the slice where it originally was. This step is critical in ensuring safety!
}

/// Scatters some elements around in an attempt to break patterns that might cause imbalanced
/// partitions in quicksort.
#[cold]
const fn break_patterns<T>(v: &mut [T]) {
  let len = v.len();
  if len >= 8 {
    // Pseudorandom number generator from the "Xorshift RNGs" paper by George Marsaglia.
    let mut random = len as u32;
    const fn gen_u32(random: &mut u32) -> u32 {
      let mut r = *random;
      r ^= r << 13;
      r ^= r >> 17;
      r ^= r << 5;
      *random = r;
      r
    }
    const fn gen_usize(random: &mut u32) -> usize {
      if usize::BITS <= 32 {
        gen_u32(random) as usize
      } else {
        (((gen_u32(random) as u64) << 32) | (gen_u32(random) as u64)) as usize
      }
    }

    // Take random numbers modulo this number.
    // The number fits into `usize` because `len` is not greater than `isize::MAX`.
    let modulus = len.next_power_of_two();

    // Some pivot candidates will be in the nearby of this index. Let's randomize them.
    let pos = len / 4 * 2;

    //for i in 0..3 {
    let mut i = 0;
    while i < 3 {
      // Generate a random number modulo `len`. However, in order to avoid costly operations
      // we first take it modulo a power of two, and then decrease by `len` until it fits
      // into the range `[0, len - 1]`.
      let mut other = gen_usize(&mut random) & (modulus - 1);

      // `other` is guaranteed to be less than `2 * len`.
      if other >= len {
        other -= len;
      }

      v.swap(pos - 1 + i, other);
      i += 1;
    }
  }
}

/// Chooses a pivot in `v` and returns the index and `true` if the slice is likely already sorted.
///
/// Elements in `v` might be reordered in the process.
const fn choose_pivot<T, F>(v: &mut [T], is_less: &mut F) -> (usize, bool)
where
  F: ~const FnMut(&T, &T) -> bool,
{
  // Minimum length to choose the median-of-medians method.
  // Shorter slices use the simple median-of-three method.
  const SHORTEST_MEDIAN_OF_MEDIANS: usize = 50;
  // Maximum number of swaps that can be performed in this function.
  const MAX_SWAPS: usize = 4 * 3;

  let len = v.len();

  // Three indices near which we are going to choose a pivot.
  let mut a = len / 4 * 1;
  let mut b = len / 4 * 2;
  let mut c = len / 4 * 3;

  // Counts the total number of swaps we are about to perform while sorting indices.
  let mut swaps = 0;

  if len >= 8 {
    // Swaps indices so that `v[a] <= v[b]`.
    // SAFETY: `len >= 8` so there are at least two elements in the neighborhoods of
    // `a`, `b` and `c`. This means the three calls to `sort_adjacent` result in
    // corresponding calls to `sort3` with valid 3-item neighborhoods around each
    // pointer, which in turn means the calls to `sort2` are done with valid
    // references. Thus the `v.get_unchecked` calls are safe, as is the `ptr::swap`
    // call.
    const fn sort2<F, T>(
      v: &mut [T],
      is_less: &mut F,
      swaps: &mut usize,
      a: &mut usize,
      b: &mut usize,
    ) where
      F: ~const FnMut(&T, &T) -> bool,
    {
      unsafe {
        if is_less(v.get_unchecked(*b), v.get_unchecked(*a)) {
          ptr::swap(a, b);
          *swaps += 1;
        }
      }
    }

    // Swaps indices so that `v[a] <= v[b] <= v[c]`.
    const fn sort3<F, T>(
      v: &mut [T],
      is_less: &mut F,
      swaps: &mut usize,
      a: &mut usize,
      b: &mut usize,
      c: &mut usize,
    ) where
      F: ~const FnMut(&T, &T) -> bool,
    {
      sort2(v, is_less, swaps, a, b);
      sort2(v, is_less, swaps, b, c);
      sort2(v, is_less, swaps, a, b);
    }

    if len >= SHORTEST_MEDIAN_OF_MEDIANS {
      // Finds the median of `v[a - 1], v[a], v[a + 1]` and stores the index into `a`.
      const fn sort_adjacent<T, F>(v: &mut [T], is_less: &mut F, swaps: &mut usize, a: &mut usize)
      where
        F: ~const FnMut(&T, &T) -> bool,
      {
        let tmp = *a;
        sort3(v, is_less, swaps, &mut (tmp - 1), a, &mut (tmp + 1));
      }

      // Find medians in the neighborhoods of `a`, `b`, and `c`.
      sort_adjacent(v, is_less, &mut swaps, &mut a);
      sort_adjacent(v, is_less, &mut swaps, &mut b);
      sort_adjacent(v, is_less, &mut swaps, &mut c);
    }

    // Find the median among `a`, `b`, and `c`.
    sort3(v, is_less, &mut swaps, &mut a, &mut b, &mut c);
  }

  if swaps < MAX_SWAPS {
    (b, swaps == 0)
  } else {
    // The maximum number of swaps was performed. Chances are the slice is descending or mostly
    // descending, so reversing will probably help sort it faster.
    v.reverse();
    (len - 1 - b, true)
  }
}

/// Sorts `v` recursively.
///
/// If the slice had a predecessor in the original array, it is specified as `pred`.
///
/// `limit` is the number of allowed imbalanced partitions before switching to `heapsort`. If zero,
/// this function will immediately switch to heapsort.
const fn recurse<'a, 'b, T, F>(
  mut v: &'a mut [T],
  is_less: &'b mut F,
  mut pred: Option<&'a T>,
  mut limit: u32,
) where
  F: ~const FnMut(&T, &T) -> bool + ~const Destruct,
{
  // Slices of up to this length get sorted using insertion sort.
  const MAX_INSERTION: usize = 20;

  // True if the last partitioning was reasonably balanced.
  let mut was_balanced = true;
  // True if the last partitioning didn't shuffle elements (the slice was already partitioned).
  let mut was_partitioned = true;

  loop {
    let len = v.len();

    // Very short slices get sorted using insertion sort.
    if len <= MAX_INSERTION {
      insertion_sort(v, is_less);
      return;
    }

    // If too many bad pivot choices were made, simply fall back to heapsort in order to
    // guarantee `O(n * log(n))` worst-case.
    if limit == 0 {
      const_heapsort(v, is_less);
      return;
    }

    // If the last partitioning was imbalanced, try breaking patterns in the slice by shuffling
    // some elements around. Hopefully we'll choose a better pivot this time.
    if !was_balanced {
      break_patterns(v);
      limit -= 1;
    }

    // Choose a pivot and try guessing whether the slice is already sorted.
    let (pivot, likely_sorted) = choose_pivot(v, is_less);

    // If the last partitioning was decently balanced and didn't shuffle elements, and if pivot
    // selection predicts the slice is likely already sorted...
    if was_balanced && was_partitioned && likely_sorted {
      // Try identifying several out-of-order elements and shifting them to correct
      // positions. If the slice ends up being completely sorted, we're done.
      if partial_insertion_sort(v, is_less) {
        return;
      }
    }

    // If the chosen pivot is equal to the predecessor, then it's the smallest element in the
    // slice. Partition the slice into elements equal to and elements greater than the pivot.
    // This case is usually hit when the slice contains many duplicate elements.
    if let Some(p) = pred {
      if !is_less(p, &v[pivot]) {
        let mid = partition_equal(v, pivot, is_less);

        // Continue sorting elements greater than the pivot.
        v = &mut v[mid..];
        continue;
      }
    }

    // Partition the slice.
    let (mid, was_p) = partition(v, pivot, is_less);
    was_balanced = cmp::min(mid, len - mid) >= len / 8;
    was_partitioned = was_p;

    // Split the slice into `left`, `pivot`, and `right`.
    let (left, right) = unsafe { split_at_mut_unchecked(v, mid) };
    let (pivot, right) = unsafe { split_at_mut_unchecked(right, 1) };
    let pivot = &pivot[0];

    // Recurse into the shorter side only in order to minimize the total number of recursive
    // calls and consume less stack space. Then just continue with the longer side (this is
    // akin to tail recursion).
    if left.len() < right.len() {
      recurse(left, is_less, pred, limit);
      v = right;
      pred = Some(pivot);
    } else {
      recurse(right, is_less, Some(pivot), limit);
      v = left;
    }
  }
}

/// Sorts `v` using pattern-defeating quicksort, which is *O*(*n* \* log(*n*)) worst-case.
pub const fn const_quicksort<'a, T, F>(v: &mut [T], mut is_less: F)
where
  F: ~const FnMut(&T, &T) -> bool + ~const Destruct,
{
  // Sorting has no meaningful behavior on zero-sized types.
  if mem::size_of::<T>() == 0 {
    return;
  }

  // Limit the number of imbalanced partitions to `floor(log2(len)) + 1`.
  let limit = usize::BITS - v.len().leading_zeros();

  recurse(v, &mut is_less, None, limit);
}
