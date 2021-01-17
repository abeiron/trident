use core::ops::{Deref, DerefMut};
use super::colour::*;
use volatile::Volatile;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char
{
  pub cp: u8,
  pub colour: ColourCode,
}

impl Deref for Char
{
  type Target = Char;

  #[inline]
  fn deref(&self) -> &Self::Target
  {
    self
  }
}

impl DerefMut for Char
{
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target
  {
    self
  }
}

pub const BUF_HEIGHT: usize = 25;
pub const BUF_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer
{
  pub chars: [[Volatile<Char>; BUF_WIDTH]; BUF_HEIGHT],
}
