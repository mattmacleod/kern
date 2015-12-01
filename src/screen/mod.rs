// use io;
static mut video_buf : *mut u16 = 0xB8000 as *mut u16;

pub fn clear() {
  let space = ' ' as u8;
  let foreground = 15; // white
  let background = 0; // black

  for x in 0..80 {
    for y in 0..25 {
      write_char(space, foreground, background, x, y)
    }
  }
}

// TODO: prevent x or y overflow?
pub fn write_char(c: u8, foreground: u8, background: u8, x: u8, y: u8) {
  let attr_byte = (background << 4) | (foreground & 0x0F);
  let character = c as u16 | ((attr_byte as u16) << 8);
  let offset = (x as u16 + (y as u16*80)) as isize;

  unsafe { 
    *video_buf.offset(offset) = character;
  }
}

pub fn write_string(s: &[u8], row: u8) {
  let foreground = 15; // white
  let background = 0; // black

  for (i, char_byte) in s.into_iter().enumerate() {
    write_char(*char_byte, foreground, background, i as u8, row);
  }
}

//   let hello = b" ";
//   let color_byte = 0x01; // white / blue

//   let mut hello_colored = [0; 80*40];
//   for (i, char_byte) in hello.into_iter().enumerate() {
//       hello_colored[i*2] = *char_byte;
//   }

//   let buffer_ptr = 0xb8000 as *mut _;
//   unsafe { *buffer_ptr = hello_colored };

// }
