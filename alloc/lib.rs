//! Allocations crate for the Trident kernel.
#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(associated_type_defaults)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(const_mut_refs)]
#![feature(core_intrinsics)]
#![feature(in_band_lifetimes)]
#![feature(lang_items)]
#![feature(wrapping_int_impl)]
#![feature(range_bounds_assert_len)]
#![cfg_attr(not(test), no_std)]

pub extern crate spin;
pub extern crate volatile;

#[macro_use]
extern crate lazy_static;
extern crate linked_list_allocator;

#[macro_use]
extern crate mopa;

/////////////////////////////////
////////// Re-exports ///////////
/////////////////////////////////

pub use core::borrow;
pub use core::cell;
pub use core::cmp;
pub use core::convert;
pub use core::fmt;
pub use core::marker;
pub use core::pin;

// END "re-exports" /////////////

/////////////////////////////////
////// Allocation routines //////
/////////////////////////////////

extern "C"
{
  /// The starting point designation for the Heap
  pub static HEAP_START: usize;

  /// The overall size of the Heap
  pub static HEAP_SIZE: usize;
}

/// Access modifiers for Volatile wrapper.
pub mod access;

pub mod alloc;
pub use self::alloc::*;

pub mod array;
pub use self::array::Array;

pub mod collections;
pub mod comp;
pub mod ctx;
pub mod engine;
pub mod entity;
pub mod env;
pub mod hash;
/*pub mod fmt;*/
pub mod math;
pub mod mmio;

pub mod string;
pub use self::string::String;
pub use self::string::StringWide;

pub mod uart;
pub mod unique;

/// For the meme.
mod boobs;
