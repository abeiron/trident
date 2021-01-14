//! Memory allocations layout.

use core::mem::{size_of, align_of};

/// Defines the layout of memory to be allocated.
pub struct Layout
{
  #[doc(hidden)]
  pub size: usize,
  #[doc(hidden)]
  pub align: usize,
}

impl Layout
{
  /// Creates a new instance of a Layout.
  #[inline]
  pub fn new(size: usize) -> Self
  {
    Layout {
      size,
      align: 4,
    }
  }

  /// Creates a new instance of a Layout from the supplied type.
  #[inline]
  pub fn from_type<T>() -> Self
  {
    Layout {
      size: size_of::<T>(),
      align: align_of::<T>(),
    }
  }

  /// Realigns data.
  #[inline]
  pub fn align_up(&self, i: usize) -> usize
  {
    let p = i + self.align - 1;
    return p - (p % self.align);
  }
}
