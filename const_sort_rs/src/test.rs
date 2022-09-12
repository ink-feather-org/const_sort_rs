pub use super::*;
mod test {
  pub use super::*;

  #[test]
  fn test_const_slice_sort() {
    const ARR: [u8; 4] = {
      let mut x = [2, 3, 5, 4];
      SliceConstUnstableSortable::const_sort_unstable(&mut x[..]);
      x
    };
    assert_eq!(&ARR, &[2, 3, 4, 5])
  }

  #[test]
  fn test_const_sort() {
    const ARR: [u8; 4] = [2, 3, 5, 4];
    const SORTED: [u8; 4] = ARR.const_sort_unstable();
    assert_eq!(&SORTED, &[2, 3, 4, 5])
  }
}
