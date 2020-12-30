//! Trident "system" crate.
//!
//! Provides a wrapper for the kernel API.
#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]

pub extern crate t_alloc as alloc;
pub extern crate t_console as console;

pub mod prelude;
pub use self::prelude::*;
