#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;

use core::fmt::Write;

mod vga_console;

#[macro_use]
mod macros;

#[no_mangle]
pub fn kmain() {
  //unsafe { write!(vga_console::WRITER, "These are some numbers: {} {}", 42, 1.337); }

  vga_console::clear();
  printk!("Hello from a new Rust kernel!\n\n");
  printk!("Here are some numbers: {} {}\n\n", 42, 1.337);
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop {} }
#[allow(non_snake_case)] #[no_mangle] pub extern "C" fn _Unwind_Resume() -> ! { loop {} }
