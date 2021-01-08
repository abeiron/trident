//! Implements a `Component` type for the E.C.E. submodule.

pub trait Component
{
  fn state(&self) -> ComponentState
  {
    ComponentState::Standby
  }
}

pub enum ComponentState
{
  Standby,
}
