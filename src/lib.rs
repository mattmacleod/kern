#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;

mod vga_console;

#[no_mangle]
pub fn kmain() {
  //unsafe { write!(vga_console::WRITER, "These are some numbers: {} {}", 42, 1.337); }

  let s = "This is a test of the VGA console code in our new Rust OS.";
  vga_console::clear();
  vga_console::write_str(s);

}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop {} }
#[allow(non_snake_case)] #[no_mangle] pub extern "C" fn _Unwind_Resume() -> ! { loop {} }
