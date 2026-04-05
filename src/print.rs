#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({use core::fmt::Write; let _ = $crate::architecture::this::WRITER.lock().write_fmt(format_args!($($arg)*));});
}
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({crate::print!($($arg)*); crate::print!("\n")};);
}
