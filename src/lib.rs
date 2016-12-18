#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;

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
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop {} }
#[allow(non_snake_case)] #[no_mangle] pub extern "C" fn _Unwind_Resume() -> ! { loop {} }
