#![crate_name="t_xkernel"]
#![crate_type="staticlib"]
#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
/*!
The Trident Exokernel
*/

// core kernel lbrary
pub(crate) extern crate t_xkernel_core as kernel;

#[no_mangle]
pub extern "C" fn kmain() -> !
{
	use kernel::console;
	console::println!("Hello, world!");

	loop {}
}
