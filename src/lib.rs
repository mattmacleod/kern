#![feature(no_std)]
#![no_std]
#![feature(lang_items, asm)]

#[no_mangle]
pub fn main() {
  unsafe {

    // Print "OKAY" to the VGA buffer
    asm!("movq rax, 0x2f592f412f4b2f4f
          mov [0xb8000], rax" :::: "intel")
  }

  loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
