#![deny(clippy::all)]
#![deny(missing_docs)]
#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(const_raw_pointer_to_usize_cast)]
#![feature(global_asm)]
#![feature(llvm_asm)]
/*!
# The Trident System
====================

Trident is an kernel system designed for performance, stability, and modularity.
*/

//=================================KERNEL ENTRY MODULE==================================//

// Lazily initialised statics
#[macro_use] extern crate lazy_static;

// Trident system crate; contains core functionality and a way to interface with the underlying hardware.
extern crate trident_sys as system;

#[doc(hidden)]
pub mod asm;

pub mod io;

#[cfg(test)]
mod test;

#[no_mangle]
pub extern "C" fn kmain() -> ! 
{
  todo!()
}
