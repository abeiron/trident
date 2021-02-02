//! Memory allocations layout.

use core::mem::{size_of, align_of};

/// Defines the layout of memory to be allocated.
#[derive(Copy, Clone)]
pub struct Layout
{
  #[doc(hidden)]
  size: usize,
  #[doc(hidden)]
  align: usize,
}

impl Layout
{
  /// Creates a new instance of a Layout.
  #[inline]
  pub fn new<T>() -> Self
  {
    Layout {
      size: size_of::<T>(),
      align: align_of::<T>(),
    }
  }

  /// Creates a new instance of a Layout from the supplied type.
  #[inline]
  pub fn from_size(size: usize) -> Self
  {
    Layout {
      size,
      align: 4,
    }
  }


  #[inline]
  pub fn from_type_array<T>(len: usize) -> Self
  {
    Self {
      size: size_of::<T>() * len,
      align: align_of::<T>(),
    }
  }

  /// Realigns data.
  #[inline(always)]
  pub fn align_up(&self, i: usize) -> usize
  {
    let p = i + self.align - 1;
    return p - (p % self.align);
  }

  #[inline]
  pub fn size(&self) -> usize
  {
    self.size
  }

  #[inline]
  pub fn align(&self) -> usize
  {
    self.align
  }
}
