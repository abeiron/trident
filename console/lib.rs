#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
#![feature(
	decl_macro,
	llvm_asm,
	panic_info_message,
)]
/*!*/

#[macro_use]
extern crate lazy_static;
extern crate volatile;

pub(crate) mod colour;
pub(crate) mod text;
pub(crate) mod write;
mod panic;

pub use self::colour::{Colour, ColourCode};
pub use self::write::{_print, print, println};
