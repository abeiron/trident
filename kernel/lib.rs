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
#[macro_use] extern crate t_boot as boot;

// Trident system crate; contains core functionality and an interface with the underlying hardware.
extern crate t_system as system;


use system::alloc::uart;

#[cfg(test)]
mod test;

/// The Program Entry Point.
///
/// All setup for the hardware's resources and the Kernel's internal workings
/// happens here.
#[no_mangle]
#[boot::entry]
pub extern "Rust" fn kmain() -> !
{
  uart::init(0x1000_0000);

  loop {
    system::console::println!("Hello world!");
  }
}
