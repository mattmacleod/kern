mod drivers {
  pub mod vga;
}

#[no_mangle]
pub fn kmain() {
  drivers::vga::clear();
  printk!("Hello from a new Rust kernel!");
}
