#![feature(no_std)]
#![no_std]
#![feature(lang_items, asm, const_fn)]

extern crate rlibc;

mod screen;

#[no_mangle]
pub fn main() {
  screen::clear();
  loop{}
}


#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
