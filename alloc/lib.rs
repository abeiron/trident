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
/*!
Allocations crate for the Trident kernel.
*/

/// Access modifiers for Volatile and VolUart wrappers.
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

pub mod boobs;
