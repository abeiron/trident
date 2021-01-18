//! String types.

//------------------------------------------------------------
//String: A growable UTF-8 string.

use crate::alloc::{Allocator, Global};
use crate::array::Array;

use core::borrow::Borrow;
use core::cmp::{Eq, PartialEq};
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ops::{DerefMut, Deref};
use core::ptr::copy_nonoverlapping;
use core::str;


/// Represents a growable UTF-8 encoded string.
pub struct String<A: Allocator = Global>
{
  buf: Array<u8, A>,
}


impl<A: Allocator> String<A>
{
  /// Initialises an empty String with the specified allocator `A`
  ///
  /// ```no_run
  /// use trident_sys::alloc::Global;
  /// use trident_sys::string::String;
  ///
  /// fn main()
  /// {
  ///   let s = String::new_with(Global);
  /// }
  /// ```
  pub fn new_with(alloc: A) -> Self
  {
    Self {
      buf: Array::new_with(alloc)
    }
  }

  /// Initialises a `String` from a given `&str` using the specified allocator, `A`.
  ///
  ///
  /// ```no_run
  /// use trident_sys::alloc::Global;
  /// use trident_sys::alloc::String;
  ///
  /// fn main()
  /// {
  ///   let s = String::from_str_with("hello!", Global);
  /// }
  /// ```
  pub fn from_str_with(s: &str, alloc: A) -> Self
  {
    let slice = s.as_bytes();
    let mut buf = Array::new_with(alloc);
    buf.resize(slice.len(), 0);

    unsafe {
      copy_nonoverlapping(s.as_ptr(), buf.as_mut_ptr(), slice.len());
    }

    Self { buf }
  }

  /// Dereferences to the base `&str`.
  #[inline]
  pub fn as_str(&self) -> &str
  {
    self
  }

  /// Pushes a Unicode character point to the current instance of `String`.
  pub fn push(&mut self, c: char)
  {
    let mut bytes = [0u8; 4];
    c.encode_utf8(&mut bytes);
    self.buf.extend(bytes[0..c.len_utf8()].iter());
  }
}

impl<A: Allocator> core::convert::TryFrom<Array<u8, A>> for String<A>
{
  type Error = core::str::Utf8Error;

  fn try_from(array: Array<u8, A>) -> Result<Self, Self::Error>
  {
    str::from_utf8(&array)?;
    Ok(Self { buf: array })
  }
}

impl String<Global>
{
  /// Initialises an empty `String`.
  ///
  ///
  /// ```
  /// use trident_sys::string::String;
  ///
  /// fn main()
  /// {
  ///   let s = String::new();
  /// }
  /// ```
  pub fn new() -> Self
  {
    Self::new_with(Global)
  }

  /// Initialises a `String` from a given `&str`.
  ///
  ///
  /// ```
  /// use trident_sys::string::String;
  ///
  /// fn main()
  /// {
  ///   let s = String::from("hello!");
  /// }
  /// ```
  pub fn from(s: &str) -> Self
  {
    Self::from_str_with(s, Global)
  }
}

impl<A: Allocator> AsRef<str> for String<A>
{
  /// References the current `String` as a `&str`.
  #[inline]
  fn as_ref(&self) -> &str
  {
    self
  }
}

impl<A: Allocator> Borrow<str> for String<A>
{
  /// Borrows the current `String` as `&str`.
  #[inline]
  fn borrow(&self) -> &str
  {
    self
  }
}

impl<A: Allocator> Deref for String<A>
{
  type Target = str;

  /// Dereferences the current `String` into a `&str`.
  ///
  ///
  /// ```
  /// use trident_sys::string::String;
  ///
  /// fn main()
  /// {
  ///   let _s = String::from("hello!");
  ///   let  s = s.deref();
  /// }
  /// ```
  #[inline]
  fn deref(&self) -> &Self::Target
  {
    unsafe {
      str::from_utf8_unchecked(&self.buf)
    }
  }
}

impl<A: Allocator> DerefMut for String<A>
{
  /// Dereferences the current `String` into a mutable `&str`.
  #[inline]
  fn deref_mut(&mut self) -> &mut str
  {
    unsafe { str::from_utf8_unchecked_mut(&mut self.buf) }
  }
}

impl<A: Allocator> fmt::Display for String<A>
{
  // allows println!() to work
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  {
    fmt::Display::fmt(self.as_str(), f)
  }
}

impl<A: Allocator> fmt::Debug for String<A>
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
  {
    fmt::Display::fmt(self.as_str(), f)
  }
}

impl<A, T> PartialEq<T> for String<A>
  where
      A: Allocator,
      T: AsRef<str>,
{
  #[inline]
  fn eq(&self, other: &T) -> bool
  {
    PartialEq::eq(self.as_str(), other.as_ref())
  }
}

impl<A: Allocator> Eq for String<A> {}

impl<A: Allocator> Hash for String<A>
{
  fn hash<H: Hasher>(&self, h: &mut H)
  {
    Hash::hash(self.as_str(), h);
  }
}

//------------------------------------------------------------
//StringWide: A growable UTF-16 string.

/// Represents a growable, UTF-16-encoded String.
pub struct StringWide<A: Allocator = Global>
{
  buf: Array<u16, A>,
}

impl<A: Allocator> StringWide<A>
{
  /// Initialises an empty `StringWide` with the specified allocator, `A`.
  ///
  ///
  /// ```no_run
  /// use trident_sys::alloc::Global;
  /// use trident_sys::string::StringWide;
  ///
  /// fn main()
  /// {
  ///   let s = StringWide::new();
  /// }
  /// ```
  pub fn new_with(alloc: A) -> Self
  {
    Self {
      buf: Array::new_with(alloc),
    }
  }

  /// Initialises a `StringWide` from the given `&str` using the specified allocator, `A`.
  ///
  ///
  /// ```no_run
  /// use trident_sys::alloc::Global;
  /// use trident_sys::string::StringWide;
  ///
  /// fn main()
  /// {
  ///   let s = StringWide::from_str_with("hello!", Global);
  /// }
  /// ```
  pub fn from_str_with(s: &str, alloc: A) -> Self
  {
    let w_iter = s.encode_utf16();

    let mut buf = Array::new_with(alloc);
    buf.reserve(w_iter.size_hint().0);

    for wchar in w_iter {
      buf.push(wchar);
    }

    Self { buf }
  }

  /// See `String::push`.
  #[inline]
  pub fn push(&mut self, c: char)
  {
    let len = c.len_utf16();
    self.buf.resize(self.buf.len() + len, 0);

    let start = self.buf.len() - len;
    c.encode_utf16(&mut self.buf[start..]);
  }
}

impl StringWide<Global>
{
  /// Initialises an empty `StringWide`.
  ///
  ///
  /// ```
  /// use trident_sys::string::StringWide;
  ///
  /// fn main()
  /// {
  ///   let s = StringWide::new();
  /// }
  /// ```
  pub fn new() -> Self
  {
    Self::new_with(Global)
  }

  /// Initialises a `StringWide` from the given `&str`.
  ///
  ///
  /// ```
  /// use trident_sys::string::StringWide;
  ///
  /// fn main()
  /// {
  ///   let s = StringWide::from("hello!");
  /// }
  /// ```
  pub fn from(s: &str) -> Self
  {
    Self::from_str_with(s, Global)
  }
}

impl<A: Allocator> AsRef<[u16]> for StringWide<A>
{
  #[inline]
  fn as_ref(&self) -> &[u16]
  {
    &self.buf
  }
}

impl<A: Allocator> Deref for StringWide<A>
{
  type Target = [u16];

  #[inline]
  fn deref(&self) -> &[u16]
  {
    &self.buf
  }
}

impl<A: Allocator> DerefMut for StringWide<A>
{
  #[inline]
  fn deref_mut(&mut self) -> &mut [u16]
  {
    &mut self.buf
  }
}

#[cfg(test)]
mod tests
{
  use super::*;
  use crate::collections::{HashMap, HashSet};
  use core::convert::TryInto;

  #[test]
  fn str()
  {
    let hello = String::from("hello");
    let mut world = String::from("world");
    world.make_ascii_uppercase();

    assert_eq!(hello.as_str(), "hello");
    assert_eq!(world.as_str(), "WORLD");
    assert_eq!(hello, "hello");
    assert_eq!(world, "WORLD");
  }

  #[test]
  fn set()
  {
    let mut set = HashSet::new();

    assert!(set.insert(String::from("hello")));
    assert!(set.insert(String::from("HELLO")));
    assert!(!set.insert(String::from("hello")));
    assert_eq!(set.len(), 2);
    assert!(set.contains("hello"));
    assert!(!set.contains("world"));
  }

  #[test]
  fn map()
  {
    let mut map = HashMap::new();

    map.insert(String::from("Hello"), 42);

    assert_eq!(map.find("Hello"), Some(&42));
    assert_eq!(map.find("world"), None);
  }

  #[test]
  fn push()
  {
    let mut s = String::new();
    s.push('a');
    s.push('é');
    s.push('漢');
    assert_eq!(s, "aé漢");
  }

  #[test]
  fn from_array()
  {
    let mut array = Array::new();
    array.extend("abé漢".bytes());

    let string: String = array.try_into().unwrap();
    assert_eq!(string, "abé漢");
  }
}
