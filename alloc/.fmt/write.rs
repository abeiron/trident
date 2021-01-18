//! Implements the `Write` trait.

use super::rt;
use super::Result;

pub trait Write
{
  fn write_str(&mut self, s: &str) -> Result;
}
