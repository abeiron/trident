use super::colour::*;
use volatile::Volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar
{
  ascii_char: u8,
  colour_code: ColourCode,
}

pub const BUF_HEIGHT: usize = 25;
pub const BUF_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer
{
  pub chars: [[Volatile<ScreenChar>; BUF_WIDTH]; BUF_HEIGHT],
}
