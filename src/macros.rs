macro_rules! printk {
    ($($arg:tt)*) => (
      use core::fmt::Write;
      write!($crate::kernel::drivers::vga::writer(), $($arg)*)
    );
}
