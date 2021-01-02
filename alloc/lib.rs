#![deny(clippy::all)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(lang_items)]
#![feature(wrapping_int_impl)]
#![cfg_attr(not(test), no_std)]

pub mod alloc;
pub mod array;
pub mod collections;
pub mod hash;
pub mod math;
pub mod string;
