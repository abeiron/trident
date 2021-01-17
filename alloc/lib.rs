#![deny(clippy::all)]
#![deny(missing_docs)]
#![allow(dead_code)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(associated_type_defaults)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(lang_items)]
#![feature(wrapping_int_impl)]
#![cfg_attr(not(test), no_std)]
/*!
Allocations crate for the Trident kernel.
*/

pub(crate) mod alloc;
pub use self::alloc::*;

pub(crate) mod array;
pub use self::array::Array;

pub mod collections;
pub mod hash;
pub mod math;

pub(crate) mod mmio;

pub(crate) mod string;
pub use self::string::String;
pub use self::string::StringWide;

pub mod uart;
