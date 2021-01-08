use core::ops::{Deref, DerefMut};
use super::colour::*;
use volatile::Volatile;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar
{
  pub ascii_char: u8,
  pub colour_code: ColourCode,
}

impl Deref for ScreenChar
{
  type Target = ScreenChar;

  #[inline]
  fn deref(&self) -> &Self::Target
  {
    self
  }
}

impl DerefMut for ScreenChar
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
  pub chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}
