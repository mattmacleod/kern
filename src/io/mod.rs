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
