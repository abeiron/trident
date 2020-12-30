#![deny(clippy::all)]
#![feature(allocator)]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_mut_refs)]
#![feature(lang_items)]
#![cfg_attr(not(test), no_std)]

pub mod alloc;
pub mod math;
