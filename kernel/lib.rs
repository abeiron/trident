#![crate_name="t_xkernel_core"]
#![crate_type="staticlib"]
#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
#![feature(
	panic_info_message, 
	llvm_asm, 
	decl_macro
)]
/*!*/

// Lazy static initialisation
#[macro_use] extern crate lazy_static;

// Phil Opp's linked list allocator for no_std projects
extern crate linked_list_allocator;

pub(crate) extern crate spin;
pub(crate) extern crate volatile;

pub mod console;
pub mod panic;
pub mod prelude;

#[cfg(test)]
mod test;
