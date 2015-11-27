#![feature(no_std)]
#![no_std]
#![feature(lang_items, start)]
#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn main() -> i64 {
    0xDEADBEEF
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
