#![deny(clippy::all)]
#![feature(lang_items)]
#![feature(const_mut_refs)]
#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]


pub mod alloc;
pub mod allocator;

extern crate alloc;
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::layout) -> ! {
	panic!("allocation error: {:?}", layout)
}