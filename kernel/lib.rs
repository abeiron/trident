#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
/*!
# The Trident System
====================

Trident is an exokernel system designed for performance, stability, and modularity.
*/

// Lazy static initialisation
#[macro_use] extern crate lazy_static;

pub(crate) extern crate t_alloc as alloc;
pub(crate) extern crate t_console as console;

pub mod io;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn kmain() -> !
{
  loop
  {
    console::println!("Hello, world!");
  }
}
