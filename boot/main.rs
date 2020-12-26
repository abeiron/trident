#![crate_name = "t_xkernel_bootloader"]
#![crate_type = "bin"]
#![feature(lang_items)]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![no_std]
#![no_main]
/*!*/

extern crate rlibc;

// The bootloader_config.rs file contains some configuration constants set by the build script:
// PHYSICAL_MEMORY_OFFSET: The offset into the virtual address space where the physical memory
// is mapped if the `map_physical_memory` feature is activated.
//
// KERNEL_STACK_ADDRESS: The virtual address of the kernel stack.
//
// KERNEL_STACK_SIZE: The number of pages in the kernel stack.
include!(concat!(env!("OUT_DIR"), "/bootloader_config.rs"));

global_asm!(include_str!("stage_1.S"));
global_asm!(include_str!("stage_2.S"));
global_asm!(include_str!("e820.S"));
global_asm!(include_str!("stage_3.S"));

#[cfg(feature = "vga_320x200")]
global_asm!(include_str!("video_mode/vga_320x200.S"));
#[cfg(not(feature = "vga_320x200"))]
global_asm!(include_str!("video_mode/vga_text_80x25.S"));
