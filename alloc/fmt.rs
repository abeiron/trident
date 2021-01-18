use core::fmt::{Arguments, Formatter, write};
use core::result;

pub type Result = result::Result<(), Error>;

#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error;

pub trait Write
{
  /// Writes a string slice into this writer, returning whether the write
  /// succeeded.
  ///
  /// This method can only succeed if the entire string slice was successfully
  /// written, and this method will not return until all data has been
  /// written or an error occurs.
  ///
  /// # Errors
  ///
  /// This function will return an instance of [`Error`] on error.
  ///
  /// # Examples
  ///
  /// ```
  /// use trident_sys::fmt::{self, Write};
  ///
  /// fn writer<W: Write>(f: &mut W, s: &str) -> fmt::Result {
  ///     f.write_str(s)
  /// }
  ///
  /// let mut buf = String::new();
  /// writer(&mut buf, "hola").unwrap();
  /// assert_eq!(&buf, "hola");
  /// ```
  fn write_str(&mut self, s: &str) -> Result;

  fn write_fmt(&mut self, args: Arguments<'_>) -> Result
  {
    todo!()
  }
}
