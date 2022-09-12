// https://doc.rust-lang.org/src/core/slice/sort.rs.html
// https://doc.rust-lang.org/src/alloc/slice.rs.html#274-276

pub trait StableSortable<T: Ord> {
  fn sort(&mut self);
}
pub trait UnstableSortable<T: Ord> {
  fn sort_unstable(&mut self);
}

impl<T: Ord> const StableSortable<T> for [T] {
  fn sort(&mut self) {
    todo!()
  }
}

impl<T: Ord> const UnstableSortable<T> for [T] {
  fn sort_unstable(&mut self) {
    todo!()
  }
}
