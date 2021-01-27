//! Implements a generic `Engine` trait from which other engines in the program can derive.

use super::context::Context;
use crate::string::String;

pub trait Engine
{
  /// Initialises the engine.
  fn init() -> Result<(), String>;

  /// Pauses the currently running operation.
  ///
  ///
  /// Returns a `Context` wrapped in a Result:
  /// Result::Ok(ctx) for success
  ///
  /// or
  ///
  /// Returns a `String` wrapped in a Result:
  /// Result::Err(err) for failure.
  ///
  /// ```
  /// use trident_sys::alloc::alloc::engine::Engine;
  /// use trident_sys::alloc::alloc::engine::EngineCtx;
  /// use trident_sys::alloc::alloc::engine::EngineState;
  /// use trident_sys::alloc::alloc::context::Context;
  ///
  /// struct Hello;
  ///
  /// impl Engine for Hello
  /// {
  ///   fn init() -> Result<(), String>
  ///   {
  ///     Ok(())
  ///   }
  ///
  ///   fn pause(&self) -> Result<dyn Context, String>
  ///   {
  ///     Ok(EngineCtx::new(EngineState::default()))
  ///   }
  ///
  ///   fn stop(self) -> Result<(), String>
  ///   {
  ///     Ok(())
  ///   }
  /// }
  /// ```
  ///
  /// [`Context`]: /alloc/context/trait.Context.html
  fn pause(&self) -> Result<dyn Context, String>;

  /// Stops the currently running operation.
  fn stop(self) -> Result<(), String>;
}

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

impl Context for EngineCtx {}
