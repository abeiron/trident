//! Trident "system" crate.
//!
//! Provides a wrapper for the kernel API.
#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]

pub extern crate t_alloc as alloc;
pub extern crate t_console as console;
pub extern crate t_core as core;

pub mod prelude;
pub use self::prelude::*;

pub mod fmt;

pub mod string;
