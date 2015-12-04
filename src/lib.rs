#![feature(no_std)]
#![no_std]
#![feature(lang_items, asm, const_fn, collections)]

extern crate rlibc;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Write;

mod screen;

#[no_mangle]
pub fn main() {
  screen::clear();
  screen::write_string(b"Kernel-mode FizzBuzz", 0);
  
  for i in 1..20 {
    let res = fizz_buzz(i);
    let s = format!("{}", res);
    screen::write_string(s.as_bytes(), (i+1) as u8);
  }
}

enum FizzBuzzResult {
  Fizz,
  Buzz,
  FizzBuzz,
  Number(i64),
}

fn fizz_buzz(i: i64) -> FizzBuzzResult {
  match (i % 3, i % 5) {
    (0, 0) => FizzBuzzResult::FizzBuzz,
    (0, _) => FizzBuzzResult::Fizz,
    (_, 0) => FizzBuzzResult::Buzz,
    _ => FizzBuzzResult::Number(i)
  }
}

impl fmt::Display for FizzBuzzResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FizzBuzzResult::Fizz => f.write_str("Fizz"),
      FizzBuzzResult::Buzz => f.write_str("Buzz"),
      FizzBuzzResult::FizzBuzz => f.write_str("FizzBuzz"),
      FizzBuzzResult::Number(4) => f.write_str("OMG it's 4"),
      FizzBuzzResult::Number(num) => write!(f, "{}", num),
    }
  }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
