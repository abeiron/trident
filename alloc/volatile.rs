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

#[derive(Copy, Clone)]
pub struct Volatile<R, A = ReadWrite>
{
  driver: Uart,
  reference: R,
  _phantom: PhantomData<A>
}

impl<R> Volatile<R, ReadWrite>
{
  pub fn new(reference: R) -> Self
  {
    Self {
      driver: Uart::new(0x1000_0000),
      reference: reference,
      _phantom: PhantomData,
    }
  }
}
