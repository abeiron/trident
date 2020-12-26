use super::colour::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub(crate) struct ScreenChar
{
  ascii_char: u8,
  colour_code: ColourCode,
}

const BUFF_HEIGHT: usize = 25;
const BUFF_WIDTH: usize = 80;

#[repr(transparent)]
pub(crate) struct Buffer
{
  chars: [[ScreenChar; BUFF_WIDTH]; BUFF_HEIGHT],
}
