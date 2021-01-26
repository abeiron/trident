use super::{colour::*, text::*};
use crate::alloc::uart::{Uart, UART_DRIVER};
use core::fmt;
use spin::Mutex;

lazy_static! {
  pub static ref GLOBAL_WRITER: Mutex<Writer> = Mutex::new(Writer
  {
    x_pos: 0,
    y_pos: 0,
    row  : 0,
    col  : 0,
    colour: ColourCode::new(Colour::Yellow, Colour::Black),
    driver: UART_DRIVER
  });
}

pub struct Writer
{
  pub x_pos : usize,
  pub y_pos : usize,
  pub row   : usize,
  pub col   : usize,
  pub colour: ColourCode,
  driver: Mutex<Uart>,
}

impl Writer
{
  pub fn write_byte(&mut self, point: u8)
  {
    match point {
      b'\n' => self.new_line(),
      point => {
        if self.x_pos >= BUF_WIDTH {
          self.new_line();
        }

        let colour = self.colour_code;

        self.driver.lock().write(Char {
          point,
          colour,
        });

        self.x_pos += 1;
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
        let character = self.driver.lock().read().unwrap();
        self.driver.lock().write(character);
      }
    }

    self.clear_row(BUF_HEIGHT - 1);
    self.col_pos = 0;
  }

  fn clear_row(&mut self, row: usize)
  {
    let blank = Char {
      point: b' ',
      colour: self.colour_code,
    };

    for col in 0..BUF_WIDTH {
      self.driver.lock().write(blank);
    }
  }

  pub fn uart() -> Uart
  {

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

/// Prints a
#[macro_export]
pub macro print
{
  ($ ($ arg: tt) *) => ($ crate::_print(format_args!($ ($ arg) *)))
}

#[macro_export]
pub macro println
{
  () => ($ crate::print!("\n")),
  ($ ($ arg: tt) *) => ($ crate::print!("{}\n", format_args!($ ($ arg) *)))
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments)
{
  use core::fmt::Write;
  UART_DRIVER.lock().write_fmt(args).unwrap();
}
