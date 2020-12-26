use core::panic::PanicInfo;

#[no_mangle] 
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! 
{
  print!("Aborting:   ");

  if let Some(p) = info.location() {
    println!(
      "line {}, file {}: {}",
      p.line(),
      p.file(),
      info.message().unwrap()
    );
  }
  else {
    print!("No information available.");
  }

  self::abort();
}

#[no_mangle]
pub extern "C" fn abort() -> !
{
  loop {
    unsafe {
      asm!("wfi"::::"volatile");
    }
  }
}
