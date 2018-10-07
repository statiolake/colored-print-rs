use std::io;
use std::io::Write;

#[cfg(unix)]
mod color_unix;

#[cfg(windows)]
mod color_windows;

#[cfg(unix)]
pub use self::color_unix::*;

#[cfg(windows)]
pub use self::color_windows::*;

#[derive(Debug, Copy, Clone)]
pub enum ConsoleColor {
    Cyan,
    Red,
    Green,
    LightGreen,
    LightMagenta,
    Yellow,
    LightBlue,
    Reset,
}

#[macro_export]
macro_rules! common_colored_print_impl {
    ($colorize:expr, $stream:expr, $color:expr, $fmt:expr $(,$args:expr)*) => (
        $crate::color::print($colorize, $stream, $color, format!($fmt $(,$args)*));
    )
}

#[macro_export]
macro_rules! common_colored_print {
    ($colorize:expr, $stream:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => (
        $(
            common_colored_print_impl!($colorize, $stream, $color, $fmt $(,$args)*);
        )*
        $crate::color::print($colorize, $stream, $crate::color::ConsoleColor::Reset, "");
    )
}

#[macro_export]
macro_rules! common_colored_println {
    ($colorize:expr, $stream:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        common_colored_print!($colorize, $stream; $($color, $fmt $(,$args)*;)*);
        $crate::color::print($colorize, $stream, $crate::color::ConsoleColor::Reset, "\n");
    };
}

#[macro_export]
macro_rules! colored_print {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => (
        common_colored_print!($colorize, $crate::Stream::Stdout; $($color, $fmt $(,$args)*;)*);
    )
}

#[macro_export]
macro_rules! colored_println {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => (
        common_colored_println!($colorize, $crate::Stream::Stdout; $($color, $fmt $(,$args)*;)*);
    )
}

#[macro_export]
macro_rules! ecolored_print {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => (
        common_colored_print!($colorize, $crate::Stream::Stderr; $($color, $fmt $(,$args)*;)*);
    )
}

#[macro_export]
macro_rules! ecolored_println {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => (
        common_colored_println!($colorize, $crate::Stream::Stderr; $($color, $fmt $(,$args)*;)*);
    )
}

use Stream;
pub fn print<S: AsRef<str>>(colorize: bool, stream: Stream, color: ConsoleColor, body: S) {
    #[cfg(unix)]
    ::color::color_unix::print(colorize, stream, color, body.as_ref());
    #[cfg(windows)]
    ::color::color_windows::print(colorize, stream, color, body.as_ref());
    io::stdout().flush().unwrap();
}
