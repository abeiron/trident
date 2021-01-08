//! Implements a `Context` type.

pub trait Context
{
  fn is_idle(&self) -> bool;
  fn is_live(&self) -> bool;
  fn is_threaded(&self) -> bool 
  {
    unimplemented!()
  }
}
