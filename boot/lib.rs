//! Bootloader for the Trident kernel

#![deny(clippy::all)]
#![warn(missing_docs)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(asm)]
#![feature(const_raw_pointer_to_unsize_cast)]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![cfg_attr(not(test), no_std)]

extern crate r0;
extern crate riscv;
extern crate t_macros;

#[doc(hidden)]
pub mod asm;

pub mod trap;

use self::trap::TrapFrame;

pub use t_macros::{entry, pre_init};

use riscv::register::mcause;

#[export_name = "error: bootloader appears more than once in the dependency graph"]
#[doc(hidden)]
pub static __ONCE__: () = ();

extern "C"
{
  // Boundaries of .bss section.
  static mut _ebss: u32;
  static mut _sbss: u32;

  // Boundaries of the .data section.
  static mut _edata: u32;
  static mut _sdata: u32;

  // Initial values in the .data section, stored in flash.
  static _sidata: u32;
}

#[export_name = "_start_rust"]
#[link_section = ".init.rust"]
pub unsafe extern "C" fn start_rust() -> !
{
  #[rustfmt::skip]
  extern "Rust"
  {
    fn kmain() -> !;

    fn __pre_init();

    fn _setup_interrupts();

    fn _mp_hook() -> bool;
  }

  if _mp_hook() {
    __pre_init();

    r0::zero_bss(&mut _sbss, &mut _ebss);
    r0::init_data(&mut _sdata, &mut _edata, &_sidata);
  }

  // TODO: Enable FPU when available.

  _setup_interrupts();

  kmain();
}

/// Trap entry point rust (_start_trap_rust)
///
/// `mcause` is read to determine the cause of the trap. XLEN-1 bit indicates
/// if it's an interrupt or an exception. The result is examined and ExceptionHandler
/// or one of the core interrupt handlers is called.
#[link_section = ".trap.rust"]
#[export_name = "_start_trap_rust"]
pub extern "C" fn start_trap_rust(trap_frame: *const TrapFrame)
{
  extern "C"
  {
    fn ExceptionHandler(trap_frame: &TrapFrame);
    fn DefaultHandler();
  }

  unsafe
      {
        let cause = mcause::read();
        if cause.is_exception() {
          ExceptionHandler(&*trap_frame)
        } else {
          let code = cause.code();
          if code < __INTERRUPTS.len() {
            let h = &__INTERRUPTS[code];
            if h.reserved == 0 {
              DefaultHandler();
            } else {
              (h.handler)();
            }
          } else {
            DefaultHandler();
          }
        }
      }
}

#[doc(hidden)]
#[no_mangle]
pub fn DefaultExceptionHandler(tf: &TrapFrame) -> !
{
  loop {
    continue;
  }
}

#[doc(hidden)]
#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub fn DefaultInterruptHandler()
{
  loop {
    // Prevent this from turning into a UDF instruction
    // see rust-lang/rust#28728 for details
    continue;
  }
}

/* Interrupts */
#[doc(hidden)]
pub enum Interrupt
{
  UserSoft,
  SupervisorSoft,
  MachineSoft,
  UserTimer,
  SupervisorTimer,
  MachineTimer,
  UserExternal,
  SupervisorExternal,
  MachineExternal,
}

pub use self::Interrupt as interrupt;

extern "C"
{
  fn UserSoft();
  fn SupervisorSoft();
  fn MachineSoft();
  fn UserTimer();
  fn SupervisorTimer();
  fn MachineTimer();
  fn UserExternal();
  fn SupervisorExternal();
  fn MachineExternal();
}

#[doc(hidden)]
pub union Vector
{
  handler: unsafe extern "C" fn(),
  reserved: usize,
}

#[doc(hidden)]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 12] = [
  Vector { handler: UserSoft },
  Vector {
    handler: SupervisorSoft,
  },
  Vector { reserved: 0 },
  Vector {
    handler: MachineSoft,
  },
  Vector { handler: UserTimer },
  Vector {
    handler: SupervisorTimer,
  },
  Vector { reserved: 0 },
  Vector {
    handler: MachineTimer,
  },
  Vector {
    handler: UserExternal,
  },
  Vector {
    handler: SupervisorExternal,
  },
  Vector { reserved: 0 },
  Vector {
    handler: MachineExternal,
  },
];

#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
pub unsafe extern "Rust" fn default_pre_init() {}

#[doc(hidden)]
#[no_mangle]
#[rustfmt::skip]
pub extern "Rust" fn default_mp_hook() -> bool
{
  use riscv::register::mhartid;
  match mhartid::read() {
    0 => true,
    _ => loop {
      unsafe { riscv::asm::wfi() }
    },
  }
}
