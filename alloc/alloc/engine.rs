//! Implements a generic `Engine` trait from which other engines in the program can derive.

use super::ctx::Context;
use crate::string::String;

pub trait Engine
{}

pub enum EngineState
{
  Running,
  Stopped,
  Idle,
  Fail { s: String },
}

impl Default for EngineState
{
  fn default() -> Self
  {
    EngineState::Idle
  }
}

pub struct EngineCtx
{
  state: EngineState,
}

impl EngineCtx
{
  pub fn new(state: EngineState) -> Self
  {
    Self { state }
  }
}

impl Context for EngineCtx
{
  fn is_idle(&self) -> bool
  {
    unimplemented!()
  }

  fn is_live(&self) -> bool
  {
    unimplemented!()
  }
}
