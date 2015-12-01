// #![feature(lang_items, asm)]

// #[inline(always)]
// pub fn out<T>(port: u16, val: T) {
//     unsafe {
//         asm!("out $1, $0" :: "{al}"(val), "{dx}"(port) :: "intel");
//     }
// }

// #[inline(always)]
// pub fn inb(port: u16) -> u8 {
//     let  val: u8;
//     unsafe {
//         asm!("in $0, $1" : "={al}"(val) : "{dx}"(port) :: "intel");
//     }
//     val
// }

pub fn clear() {
  let hello = b"Hello Altmetric!";
  let color_byte = 0x1a; // white / blue

  let mut hello_colored = [color_byte; 80 * 40];
  for (i, char_byte) in hello.into_iter().enumerate() {
      hello_colored[i*2] = *char_byte;
  }

  let buffer_ptr = 0xb8000 as *mut _;
  unsafe { *buffer_ptr = hello_colored };

}
