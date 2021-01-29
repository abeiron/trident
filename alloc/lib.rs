//! Allocations crate for the Trident kernel.
#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(associated_type_defaults)]
#![feature(coerce_unsized)]
#![feature(const_fn)]
#![feature(const_generics)]
#![feature(const_mut_refs)]
#![feature(core_intrinsics)]
#![feature(decl_macro)]
#![feature(in_band_lifetimes)]
#![feature(lang_items)]
#![feature(proc_macro_hygiene)]
#![feature(range_bounds_assert_len)]
#![feature(unsize)]
#![feature(wrapping_int_impl)]
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

pub use core::any;
pub use core::borrow;
pub use core::cell;
pub use core::cmp;
pub use core::convert;
pub use core::fmt;
pub use core::marker;
pub use core::pin;

pub use self::alloc::*;
pub use self::array::Array;
pub use self::string::String;
pub use self::string::StringWide;

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

// END "allocation routines" ////

pub mod alloc;
pub mod array;
pub mod bitflags;
pub mod collections;
pub mod env;
pub mod hash;
/*pub mod fmt;*/
pub mod mmio;
pub mod ptr;
pub mod string;
pub mod uart;
pub mod volatile;

/// For the meme.
mod boobs;
