//! The `Volatile` type is a wrapper that provides volatile read/write access to memory without
//! having to delve into `unsafe` territory.

use crate::{
  marker::PhantomData,
  spin::Mutex,
  uart::Uart,
};

use core::ops::Deref;


/// Access modifiers for Volatile wrapper.
pub mod access;
use access::*;

/// The `Volatile` wrapper type enables convenient and efficient "volatile" writes.
#[derive(Copy, Clone)]
pub struct Volatile<R, A = ReadWrite>
{
  driver: Uart,
  reference: R,
  _phantom: PhantomData<A>
}

impl<R> Volatile<R, ReadWrite>
{
  /// Creates a new "volatile" wrapper instance.
  /// 
  /// # Examples
  /// 
  /// ```
  /// let v = Volatile::new(&0);
  /// ```
  pub fn new(reference: R) -> Self
  {
    Self {
      driver: Uart::new(0x1000_0000),
      reference: reference,
      _phantom: PhantomData,
    }
  }

  /// Creates a read-only instance of `Volatile`.
  /// 
  /// # Examples
  /// 
  /// ```
  /// let ro = Volatile::new_read_only(&0);
  /// ```
  pub fn new_read_only(reference: R) -> Volatile<R, ReadOnly>
  where
      A: Readable,
  {
    Self {
      driver: Uart::new(0x1000_0000),
      reference: reference,
      _phantom: PhantomData,
    }
  }

  /// Creates a write-only instance of `Volatile`.
  /// 
  /// # Examples
  /// 
  /// ```
  /// let wo = Volatile::new_write_only(&0);
  /// ```
  pub fn new_write_only(reference: R) -> Volatile<R, WriteOnly>
  where
      A: Writeable,
  {
    Self {
      driver: Uart::new(0x1000_0000),
      reference: reference,
      _phantom: PhantomData,
    }
  }
}

impl<R, T, A> Volatile<R, A>
where
    R: Deref<Target = T>,
    T: Copy,
{
  pub fn read(&self) -> Option<T>
  where
      A: Readable,
  {
    self.driver.read()
  }
}
