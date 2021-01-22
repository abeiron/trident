//! Bootloader for the Trident kernel

#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![cfg_attr(not(test), no_std)]

#[doc(hidden)]
pub mod asm;
