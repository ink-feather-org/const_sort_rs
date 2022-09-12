use alloc::slice;
/// # Safety
///
/// Calling this method with an out-of-bounds index is *[undefined behavior]*
/// even if the resulting reference is not used. The caller has to ensure that
/// `0 <= mid <= self.len()`.
///
/// [`split_at_mut`]: slice::split_at_mut
/// [undefined behavior]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
pub const unsafe fn split_at_mut_unchecked<T>(selv: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
  // from https://doc.rust-lang.org/src/core/slice/mod.rs.html#1669-1681
  let len = selv.len();
  let ptr = selv.as_mut_ptr();
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
