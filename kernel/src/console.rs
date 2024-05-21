use core::fmt::{self, Write};
use sbi::legacy::sbi_console_putchar;
struct Stdout;

impl Write for Stdout {
    /// Writes a string slice to the console.
    ///
    /// # Arguments
    ///
    /// * `s` - The string slice to be written.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the write operation is successful.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u8; 4];
        for c in s.chars() {
            if c.is_ascii() {
                sbi_console_putchar(c as u8);
            } else {
                for &code in c.encode_utf8(&mut buffer).as_bytes() {
                    sbi_console_putchar(code);
                }
            }
        }
        Ok(())
    }
}

/// Print a formatted string to the console.
///
/// You may not need to use this function directly.
/// Instead, you can use the `print!` and `println!` macros.
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print a formatted string to the console, like the one in the standard library.
///
/// # Example
/// ```
/// print!("Hello, world!");            // Print "Hello, world!"
/// print!("Hello, {}", "world!");      // Same as above
/// print!("你好，{}", "世界！");          // Print "你好，世界！"
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::print(format_args!($($arg)*)));
}

/// Print a formatted string to the console, with a newline at the end.
///
/// # Example
/// ```
/// println!();                         // Print a newline
/// println!("Hello, world!");          // Print "Hello, world!" and a newline
/// println!("Hello, {}", "world!");    // Same as above
/// ```
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
