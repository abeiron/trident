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

/////////////////////////////////
////// Allocation routines //////
/////////////////////////////////

extern "C"
{
  pub static HEAP_START: usize;
  pub static HEAP_SIZE: usize;
}

/// Access modifiers for Volatile wrapper.
pub(crate) mod access;

pub(crate) mod alloc;
pub use self::alloc::*;

pub(crate) mod array;
pub use self::array::Array;

pub mod collections;
pub mod hash;
pub mod fmt;
pub mod math;

pub(crate) mod mmio;

pub(crate) mod string;
pub use self::string::String;
pub use self::string::StringWide;

pub mod uart;
pub(crate) mod volatile;
pub use self::volatile::Volatile;

// For the meme.
pub mod boobs;
