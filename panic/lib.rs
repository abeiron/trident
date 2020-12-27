#![deny(clippy::all)]
#![no_std]
#![features(
  panic_info_message, 
  llvm_asm,
)]

extern crate t_console as console;

use core::panic::PanicInfo;

#[no_mangle] 
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
  console::print!("Aborting:   ");

  if let Some(p) = info.location() {
    console::println!(
      "line {}, file {}: {}",
      p.line(),
      p.file(),
      info.message().unwrap()
    );
  }
  else {
    console::print!("No information available.");
  }

  self::abort();
}

#[no_mangle]
pub extern "C" fn abort() -> !
{
  loop {
    unsafe {
      llvm_asm!("wfi"::::"volatile");
    }
  }
}
