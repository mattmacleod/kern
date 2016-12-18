macro_rules! printk {
    ($($arg:tt)*) => (write!(vga_console::writer(), $($arg)*));
}
