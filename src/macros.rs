use core::fmt::Write;

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    ($fmt:expr, $($arg:tt)*) => ({
        use core::fmt::Write;
        let mut vga = $crate::Context.vga.lock();
        vga.write_fmt(format_args!($($arg)*)).unwrap();
    });
    ($fmt:expr) => ({
        let mut vga = $crate::Context.vga.lock();
        vga.write($fmt);
    });
}