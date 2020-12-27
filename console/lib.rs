#![deny(clippy::all)]
#![no_std]
#![feature(
	decl_macro
)]
/*!*/

#[macro_use]
extern crate lazy_static;
extern crate volatile;

pub(crate) mod colour;
pub(crate) mod text;
pub(crate) mod write;

pub use self::colour::{Colour, ColourCode};
pub use self::write::{_print, print, println};