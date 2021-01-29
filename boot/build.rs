// NOTE: Adapted from cortex-m/build.rs
extern crate riscv_target;

use riscv_target::Target;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

fn main()
{
  let target = env::var("TARGET").unwrap();
  let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let name = env::var("CARGO_PKG_NAME").unwrap();

  if target.starts_with("riscv") {
    let mut target = Target::from_target_str(&target);
    target.retain_extensions("imc");

    let target = target.to_string();

    fs::copy(
      format!("bin/{}.a", target),
      out_dir.join(format!("lib{}.a", name)),
    )
        .unwrap();

    println!("cargo:rustc-link-lib=static={}", name);
    println!("cargo:rustc-link-search={}", out_dir.display());
  }

  let mut fi = File::create(&out_dir.join("memory.ld"))
      .expect("could not create file");

  fi.write_all(include_bytes!("scripts/memory.ld"))
      .expect("could not write file");

  // Put the linker script somewhere the linker can find it
  File::create(out_dir.join("link.ld"))
      .unwrap()
      .write_all(include_bytes!("scripts/link.ld"))
      .unwrap();

  File::create(out_dir.join("virt.lds"))
      .unwrap()
      .write_all(include_bytes!("scripts/virt.lds"))
      .unwrap();

  println!("cargo:rustc-link-search={}", out_dir.display());

  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=link.ld");
}
