pub use super::*;
mod test {
  pub use super::*;

  #[test]
  fn test_mut_ref_sort() {
    const arr: &[u8] = &[2, 3, 5, 4];
    (arr as mut_ref_sort::StableSortable).sort();
  }
}
