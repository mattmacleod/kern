#![feature(lang_items, const_fn, unique)]
#![no_std]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[macro_use]
mod macros;
pub mod kernel;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop {} }
#[allow(non_snake_case)] #[no_mangle] pub extern "C" fn _Unwind_Resume() -> ! { loop {} }
