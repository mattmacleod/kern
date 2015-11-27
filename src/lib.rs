#![feature(no_std)]
#![no_std]
#![feature(lang_items, asm)]

extern crate rlibc;

#[no_mangle]
pub extern fn main() -> i64 {
  loop{}
}

// static cursor_y: u8 = 0;
// static cursor_x: u8 = 0;

// fn move_cursor()
// {
//    let cursor_location: u16 = cursor_y as u16 * 80 + cursor_x as u16;

//    // High byte
//    outb(0x3D4, 14);
//    outb(0x3D5, (cursor_location >> 8) as u8);

//    // Low byte
//    outb(0x3D4, 15);
//    outb(0x3D5, (cursor_location) as u8);
// }

// #[inline(always)]
// pub fn out<T>(port: u16, val: T) {
//     unsafe {
//         asm!("out $1, $0" :: "{al}"(val), "{dx}"(port) :: "intel");
//     }
// }

// #[inline(always)]
// pub fn inb(port: u16) -> u8 {
//     let val: u8;
//     unsafe {
//         asm!("in $0, $1" : "={al}"(val) : "{dx}"(port) :: "intel");
//     }
//     val
// }

// fn inw(port: u16) -> u16
// {
//    let ret: u16;
//    unsafe { asm!("inb %1, %0" : "=a" (ret) : "dN" (port)); }
//    ret
// }


#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
