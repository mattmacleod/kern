#![feature(no_std, lang_items, const_fn, unique, core_str_ext)]
#![no_std]

extern crate rlibc;

use core::fmt::Write;

mod vga_console;

#[no_mangle]
pub fn kmain() {
  // vga_console::clear();

  // unsafe { write!(vga_console::WRITER, "These are some numbers: {} {}", 42, 1.337); }
  let s = "test";
  unsafe { vga_console::WRITER.write_str(s); }
  // unsafe { vga_console::WRITER.write_str("MattOS"); }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
 
