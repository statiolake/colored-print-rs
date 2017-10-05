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

pub enum ConsoleColor {
    Cyan,
    Red,
    Green,
    LightMagenta,
    Yellow,
    LightBlue,
    Reset,
}

#[macro_export]
macro_rules! colored_print_impl {
    ($colorize:expr, $c:expr, $fmt:expr) => (
        $crate::color::print($colorize, $c, $fmt);
    );
    ($colorize:expr, $c:expr, $fmt:expr, $($args:expr)*) => (
        $crate::color::print($colorize, $c, format!($fmt, $($args)*));
    )
}

#[macro_export]
macro_rules! colored_print {
    ($colorize:expr; $($c:expr, $fmt:expr, $($args:expr)*);+;) => (
        $(
            colored_print_impl!($colorize, $c, $fmt, $($args)*);
        )*
        $crate::color::print($colorize, $crate::color::ConsoleColor::Reset, "");
    )
}

#[macro_export]
macro_rules! colored_println {
    ($colorize:expr; $($c:expr, $fmt:expr, $($args:expr)*);+;) => (
        $(
            colored_print_impl!($colorize, $c, $fmt, $($args)*);
        )*
        $crate::color::print($colorize, $crate::color::ConsoleColor::Reset, "\n");
    )
}

pub fn print<S: AsRef<str>>(colorize: bool, kind: ConsoleColor, body: S) {
    if colorize {
        print!("{}{}{}", kind, body.as_ref(), ::color::ConsoleColor::Reset);
    } else {
        print!("{}", body.as_ref());
    }
    io::stdout().flush().unwrap();
}
