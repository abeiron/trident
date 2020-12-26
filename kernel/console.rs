mod colour;
mod text;
mod write;

pub use { 
	self::colour::{ Colour, ColourCode },
	self::write::{_print, print, println },
};
