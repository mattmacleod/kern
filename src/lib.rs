#![feature(no_std)]
#![no_std]
#![feature(lang_items, asm, const_fn)]

extern crate rlibc;

#[no_mangle]
pub fn main() {

  let hello = b"Hello Altmetric!";
  let color_byte = 0x1a; // white / blue

  let mut hello_colored = [color_byte; 80 * 40];
  for (i, char_byte) in hello.into_iter().enumerate() {
      hello_colored[i*2] = *char_byte;
  }

  let buffer_ptr = 0xb8000 as *mut _;
  unsafe { *buffer_ptr = hello_colored };

  loop{}
}


#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
