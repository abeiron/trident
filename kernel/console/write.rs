use core::fmt;
use spin::Mutex;
use super::{colour::*, text::*};

lazy_static! {
  pub static ref GLOBAL_WRITER: Mutex<Writer> = Mutex::new(Writer {
    col_pos: 0,
    colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
    buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}

pub struct Writer
{
  col_pos: usize,
  colour_code: ColourCode,
  buf: &'static mut Buffer,
}

impl Writer
{
  pub fn write_byte(&mut self, byte: u8)
  {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.col_pos >= BUFF_WIDTH {
          self.new_line();
        }

        let row = BUFF_HEIGHT - 1;
        let col = self.col_pos;

        let colour_code = self.colour_code;
        self.buffer.chars[row][col].write(ScreenChar {
          ascii_char: byte,
          colour_code,
        });

        self.col_pos += 1;
      }
    }
  }

  pub fn write_string(&mut self, s: &str)
  {
    for byte in s.bytes() {
      match byte {
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        // Not part of printable byte range.
        _ => self.write_byte(0xfe),
      }
    }
  }

  fn new_line(&mut self) 
  {
    for row in 1..BUFF_HEIGHT {
      for col in 0..BUFF_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }

    self.clear_row(BUFF_HEIGHT - 1);
    self.col_pos = 0;
  }

  fn clear_row(&mut self, row: usize)
  {
    let blank = ScreenChar {
      ascii_char: b' ',
      colour_code: self.colour_code,
    };
    for col in 0..BUFF_WIDTH {
      self.buffer.chars[row][col].write(blank);
    }
  }
}

impl fmt::Write for Writer
{
  fn write_str(&mut self, s: &str) -> fmt::Result<()>
  {
    self.write_string(s);


    Ok(())
  }
}

macro_rules! print 
{
  ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

macro_rules! println 
{
  () => ($crate::console::print!("\n"));
  ($($arg::tt)*) => ($crate::console::print!("{}\n", format_args($(arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
  use core::fmt::Write;
  GLOBAL_WRITER.lock().write_fmt(args).unwrap();
}
