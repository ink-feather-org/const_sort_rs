pub use super::*;
mod test {
  pub use super::*;

  #[test]
  fn test_const_sort() {
    const ARR: [u8; 4] = [2, 3, 5, 4];
    const SORTED: [u8; 4] = ARR.const_sort();
    assert_eq!(&SORTED, &[2, 3, 4, 5])
  }

  #[test]
  fn test_const_sort_stable_check() {
    #[derive(Debug)]
    struct StabilityTest(u8, u8);
    impl const PartialEq for StabilityTest {
      fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
      }
    }
    impl const PartialOrd for StabilityTest {
      fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
      }
    }
    const ARR: [StabilityTest; 4] = [
      StabilityTest(2, 1),
      StabilityTest(2, 2),
      StabilityTest(3, 1),
      StabilityTest(2, 3),
    ];
    const SORTED: [StabilityTest; 4] = ARR.const_sort();
    assert_eq!(
      &SORTED,
      &[
        StabilityTest(2, 1),
        StabilityTest(2, 2),
        StabilityTest(2, 3),
        StabilityTest(3, 1),
      ]
    );
  }
}
