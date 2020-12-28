#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
#![feature(decl_macro)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
/*!
# The Trident System
====================

Trident is an exokernel system designed for performance, stability, and modularity.
*/

// Lazy static initialisation
#[macro_use] extern crate lazy_static;

// Phil Opp's linked list allocator for no_std projects
extern crate linked_list_allocator;

pub(crate) extern crate t_alloc as _alloc;
pub(crate) extern crate t_console as _console;
pub(crate) extern crate spin;
pub(crate) extern crate volatile;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn kmain() -> !
{
  loop {}
}
