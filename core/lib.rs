//! The Core library for the Trident kernel system.

#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![cfg_attr(not(test), no_std)]

extern crate t_alloc as alloc;
pub extern crate backtracer as bt;

//--------------------------------------------------------------------------------------------------
// Modules /////////////////////////////////////////////////////////////////////////////////////////
//--------------------------------------------------------------------------------------------------

pub mod error;
pub mod math;
