use super::{colour::*, text::*};
use core::fmt;
use spin::Mutex;

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
        if self.col_pos >= BUF_WIDTH {
          self.new_line();
        }

        let row = BUF_HEIGHT - 1;
        let col = self.col_pos;

        let colour_code = self.colour_code;
        self.buf.chars[row][col].write(Char {
          cp: byte,
          colour,
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
    for row in 1..BUF_HEIGHT {
      for col in 0..BUF_WIDTH {
        let character = self.buf.chars[row][col].read();
        self.buf.chars[row - 1][col].write(character);
      }
    }

    self.clear_row(BUF_HEIGHT - 1);
    self.col_pos = 0;
  }

  fn clear_row(&mut self, row: usize)
  {
    let blank = Char {
      cp: b' ',
      colour: self.colour_code,
    };

    for col in 0..BUF_WIDTH {
      self.buf.chars[row][col].write(blank);
    }
  }
}

impl fmt::Write for Writer
{
  fn write_str(&mut self, s: &str) -> fmt::Result
  {
    self.write_string(s);


    Ok(())
  }
}

pub macro print 
{
  ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)))
}

pub macro println 
{
  () => ($crate::print!("\n")),
  ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)))
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
  use core::fmt::Write;
  GLOBAL_WRITER.lock().write_fmt(args).unwrap();
}
