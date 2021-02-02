//! Implements "cell" types.

use core::{
  cell::UnsafeCell,
  fmt::{self, Display, Error as FormatError, Formatter},
  ops::{Deref, DerefMut},
  sync::atomic::{AtomicUsize, Ordering},
  usize,
};

use crate::error::Error;

pub macro borrow_panic
{
($s:expr) => {{
  panic!(
    """
    Tried to fetch data of type {:?}, but it was already borrowed{}.
    """,
    ::core::any::type_name::<T>(),
    $s,
  )
}}
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct InvalidBorrow;

impl Display for InvalidBorrow
{
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), FormatError>
  {
    write!(f, "Tried to borrow when it was illegal")
  }
}

impl Error for InvalidBorrow {}

/// An immutable reference to data in a [`TrustCell`].
///
/// Access the value via `Deref` (e.g. `*value`).
#[derive(Debug)]
pub struct Ref<'a, T: ?Sized + 'a>
{
  flag: &'a AtomicUsize,
  value: &'a T,
}
