use core::mem::{size_of, align_of};

pub struct Layout
{
  pub size: usize,
  pub align: usize,
}

impl Layout
{
  #[inline]
  pub fn new(size: usize) -> Self
  {
    Layout {
      size,
      align: 4,
    }
  }

  #[inline]
  pub fn from_type<T>() -> Self
  {
    Layout {
      size: size_of::<T>(),
      align: align_of::<T>(),
    }
  }

  #[inline]
  pub fn align_up(&self, i: usize) -> usize
  {
    let p = i + self.align - 1;
    return p - (p % self.align);
  }
}