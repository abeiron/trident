/*!
# The Trident System
====================

Trident is a kernel system designed for performance, stability, and modularity.
*/

#![deny(clippy::all)]
#![warn(missing_docs)]
#![cfg_attr(not(test), no_std)]

//=================================KERNEL ENTRY MODULE==================================//

// Lazily initialised statics
#[macro_use] extern crate lazy_static;

// Trident system bootloader.
extern crate t_boot as boot;

// Trident system crate; contains core functionality and an interface with the underlying hardware.
extern crate t_system as system;

pub mod io;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "Rust" fn kmain() -> !
{
  loop {
    system::console::println!("Hello world!");
  }
}
