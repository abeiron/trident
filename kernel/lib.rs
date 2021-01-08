/*!
# The Trident System
====================

Trident is an exokernel system designed for performance, stability, and modularity.
*/
#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]

//=================================KERNEL ENTRY MODULE==================================//

// Lazily initialised statics
#[macro_use] extern crate lazy_static;

// Trident system crate; contains core funtionality and a way to interface with the underlying hardware.
extern crate trident_sys as system;

pub mod io;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn kmain() -> !
{
  use system::alloc;
  use system::console;

  alloc::init_heap();

  loop {
    console::println!("Hello, world!");
  }
}
