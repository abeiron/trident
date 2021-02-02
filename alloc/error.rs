use core::{
  any::TypeId,
  fmt::{self, Debug, Display},
};

use crate::{
  string::{String, StringWide},
  unique::Unq
};

pub trait Error: Debug + Display
{
  /// The lower-level source of this error, if any.
  ///
  /// # Examples
  ///
  /// ```compile_fail
  /// use t_system::error::Error;
  /// use core::fmt;
  ///
  /// #[derive(Debug)]
  /// struct SuperError
  /// {
  ///   side: SuperErrorSideKick,
  /// }
  ///
  /// impl fmt::Display for SuperError
  /// {
  ///   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  ///   {
  ///     write!(f, "SuperError is here!")
  ///   }
  /// }
  ///
  /// impl Error for SuperError
  /// {
  ///   fn source(&self) -> Option<&(dyn Error + 'static)>
  ///   {
  ///     Some(&self.side)
  ///   }
  /// }
  ///
  /// #[derive(Debug)]
  /// struct SuperErrorSideKick;
  ///
  /// impl fmt::Display for SuperErrorSideKick
  /// {
  ///   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  ///   {
  ///     write!(f, "SuperErrorSideKick is here!")
  ///   }
  /// }
  ///
  /// impl Error for SuperErrorSideKick {}
  ///
  /// fn get_super_error() -> Result<(), SuperError>
  /// {
  ///   Err(SuperError { side: SuperErrorSideKick })
  /// }
  ///
  /// fn main()
  /// {
  ///   match get_super_error() {
  ///     Err(e) => {
  ///       println!("Error: {}", e);
  ///       println!("Caused by: {}", e.source().unwrap());
  ///     }
  ///     _ => println!("No error."),
  ///   }
  /// }
  /// ```
  fn source(&self) -> Option<&(dyn Error + 'static)>
  {
    None
  }

  /// Gets the `TypeId` of `self`.
  fn type_id(&self, _: private::Internal) -> TypeId
    where
        Self: 'static,
  {
    TypeId::of::<Self>()
  }

  /// Returns a stack backtrace, if one is available, where this error occurred.
  ///
  /// This function allows inspecting the location in code where an error
  /// happened. The returned `Backtrace` contains information about the stack
  /// trace of the system thread of execution of where the error originated from.
  ///
  /// Note that not all errors contain a `Backtrace`.
  /// Also note that a `Backtrace` can be empty.
  fn backtrace(&self) -> Option<()>
  {
    todo!("Implement a backtrace function");
    None
  }
}

mod private
{
  /// A hack to prevent `type_id` from being overridden by `Error`
  /// implementations, since that may enable unsound downcasting.
  #[derive(Debug)]
  pub struct Internal;
}

impl<'a, E: Error + 'a> From<E> for Unq<dyn Error + 'a>
{
  /// Converts a type of [`Error`] into a unique pointer of dyn [`Error`].
  ///
  /// # Examples
  ///
  /// ```compile_fail
  /// use t_system::error::Error;
  /// use t_system::unique::Unq;
  /// use core::fmt;
  /// use core::mem;
  ///
  /// #[derive(Debug)]
  /// struct AnError;
  ///
  /// impl fmt::Display for AnError
  /// {
  ///   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
  ///   {
  ///     write!(f, "An error")
  ///   }
  /// }
  ///
  /// impl Error for AnError {}
  ///
  /// let an_error = AnError;
  /// assert!(0 == mem::size_of_val(&an_error);
  /// let a_unique_error = Unq::<dyn Error>::from(an_error);
  /// assert!(mem::size_of::<Unq<dyn Error>>() == mem::size_of_val(&a_unique_error));
  /// ```
  fn from(err: E) -> Unq<dyn Error + 'a>
  {
    Unq::new(err)
  }
}

impl<'a, E: Error + Send + Sync + 'a> From<E> for Unq<dyn Error + Send + Sync + 'a>
{
  /// Converts a type of [`Error`] + [`Send`] + [`Sync`] into a unique pointer of
  /// dyn [`Error`] + [`Send`] + [`Sync`].
  fn from(err: E) -> Unq<dyn Error + Send + Sync + 'a>
  {
    Unq::new(err)
  }
}

impl From<String> for Unq<dyn Error + Send + Sync>
{
  /// Converts a [`String`] into a unique pointer of
  /// dyn [`Error`] + [`Send`] + [`Sync`].
  ///
  /// # Examples
  ///
  /// ```compile_fail
  /// use t_system::error::Error;
  /// use t_system::string::String;
  /// use t_system::unique::Unq;
  ///
  /// let a_string_err = String::from("A string error.");
  /// let a_unique_err = Unq::<dyn Error + Send + Sync>::from(a_string_err);
  /// ```
  fn from(err: String) -> Unq<dyn Error + Send + Sync>
  {
    struct StringErr(String);

    impl Error for StringErr {}

    impl Display for StringErr
    {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
      {
        Display::fmt(&self.0, f)
      }
    }

    impl Debug for StringErr
    {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
      {
        Debug::fmt(&self.0, f)
      }
    }

    Unq::new(StringErr(err))
  }
}

impl From<String> for Unq<dyn Error>
{
  /// Converts a [`String`] into a unique pointer of
  /// dyn [`Error`].
  fn from(err: String) -> Unq<dyn Error>
  {
    let e: Unq<dyn Error + Send + Sync> = From::from(err);
    let h: Unq<dyn Error> = e;

    h
  }
}

impl<'a> From<&str> for Unq<dyn Error + Send + Sync + 'a>
{
  /// Converts a [`str`] into a unique pointer of dyn [`Error`] + [`Send`] + [`Sync`].
  ///
  /// [`str`]: prim@str
  ///
  /// # Examples
  ///
  /// ```compile_fail
  /// use t_system::error::Error;
  /// use t_system::unique::Unq;
  ///
  /// let a_str_error = "a str error";
  /// let a_unique_error = Unq::<dyn Error + Send + Sync>::from(a_str_error);
  /// ```
  #[inline]
  fn from(err: &str) -> Unq<dyn Error + Send + Sync + 'a>
  {
    From::from(String::from(err))
  }
}

impl From<&str> for Unq<dyn Error>
{
  /// Converts a [`str`] into a unique pointer of dyn [`Error`].
  ///
  /// [`str`]: prim@str
  fn from(err: &str) -> Unq<dyn Error>
  {
    From::from(String::from(err))
  }
}

use crate::alloc::AllocErr;
impl Error for AllocErr {}

use crate::alloc::layout::LayoutErr;
impl Error for LayoutErr {}
