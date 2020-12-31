#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
/*!
# The Trident System
====================

Trident is an exokernel system designed for performance, stability, and modularity.
*/

// Lazy static initialisation
#[macro_use] extern crate lazy_static;

// Phil Opp's linked list allocator for no_std projects
extern crate linked_list_allocator;

pub(crate) extern crate t_alloc as alloc;
pub(crate) extern crate t_console as console;
pub(crate) extern crate spin;
pub(crate) extern crate volatile;

pub mod io;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn kmain() -> !
{
  loop {
    console::println!("Hello, world!");
  }
}
