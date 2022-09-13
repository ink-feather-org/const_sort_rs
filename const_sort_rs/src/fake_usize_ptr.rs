use core::cmp::Ordering;
use core::ops::{Index, IndexMut};

#[derive(Eq, Clone, Copy)]
pub(crate) struct FakeUsizePtr(usize);
impl FakeUsizePtr {
  pub const fn null_mut() -> Self {
    Self(0)
  }
  // pub const fn offset(self, count: isize) -> Self {
  //   Self(self.0.checked_add_signed(count).unwrap())
  // }
  pub const fn add(self, count: usize) -> Self {
    Self(self.0.checked_add(count).unwrap())
  }
  pub const fn sub(self, count: usize) -> Self {
    Self(self.0.checked_sub(count).unwrap())
  }
  pub const fn addr(self) -> usize {
    self.0
  }
}

impl<T> const Index<FakeUsizePtr> for [T] {
  type Output = T;

  fn index(&self, index: FakeUsizePtr) -> &Self::Output {
    &self[index.0]
  }
}

impl<T> const IndexMut<FakeUsizePtr> for [T] {
  fn index_mut(&mut self, index: FakeUsizePtr) -> &mut Self::Output {
    &mut self[index.0]
  }
}

impl const PartialEq for FakeUsizePtr {
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}

impl const PartialOrd for FakeUsizePtr {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl const Ord for FakeUsizePtr {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}
