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
