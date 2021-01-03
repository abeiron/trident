#![deny(clippy::all)]
#![deny(missing_docs)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(lang_items)]
#![feature(wrapping_int_impl)]
#![cfg_attr(not(test), no_std)]
/*!
Allocations crate for the Trident kernel.
*/

pub mod alloc;
pub use self::alloc::Global;
pub use self::alloc::Layout;
pub use self::alloc::Locked;

pub mod array;
pub use self::array::Array;

pub mod collections;
pub mod hash;
pub mod math;

pub mod string;
pub use self::string::String;
pub use self::string::StringWide;
