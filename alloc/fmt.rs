//! Utilities for formatting and printing `String`s and `StringWide`s.
//! 
//! [`String`]: /string/struct.String.html
//! [`StringWide`]: /string/struct.StringWide.html

mod error;
pub use self::error::Error;

mod rt;
mod read;
pub use self::read::Read;

mod result;
pub use self::result::Result;

mod write;
pub use self::write::Write;
