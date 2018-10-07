use winapi::shared::minwindef::WORD;
use winapi::um::winnt::HANDLE;
use winapi::um::{processenv, winbase, wincon};

use std::io;
use std::io::prelude::*;

use super::{ConsoleColor as CC, Stream};

const RAW_CYAN: WORD = wincon::FOREGROUND_BLUE | wincon::FOREGROUND_GREEN;
const RAW_RED: WORD = wincon::FOREGROUND_RED;
const RAW_GREEN: WORD = wincon::FOREGROUND_GREEN;
const RAW_LIGHT_GREEN: WORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_INTENSITY;
const RAW_LIGHT_MAGENTA: WORD =
    wincon::FOREGROUND_BLUE | wincon::FOREGROUND_RED | wincon::FOREGROUND_INTENSITY;
const RAW_YELLOW: WORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_RED;
const RAW_LIGHT_BLUE: WORD = wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY;
const RAW_RESET: WORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE | wincon::FOREGROUND_RED;

pub fn print(colorize: bool, stream: Stream, color: CC, body: &str) {
    io::stdout().flush().unwrap();
    io::stderr().flush().unwrap();
    let handle = get_stream_handle(stream);
    set_console_color(colorize, handle, color);
    match stream {
        Stream::Stdout => write!(io::stdout(), "{}", body).unwrap(),
        Stream::Stderr => write!(io::stderr(), "{}", body).unwrap(),
    }
    io::stdout().flush().unwrap();
    io::stderr().flush().unwrap();
    set_console_color(colorize, handle, CC::Reset);
}

fn get_stream_handle(stream: Stream) -> HANDLE {
    let raw_stream = match stream {
        Stream::Stdout => winbase::STD_OUTPUT_HANDLE,
        Stream::Stderr => winbase::STD_ERROR_HANDLE,
    };
    unsafe { processenv::GetStdHandle(raw_stream) }
}

fn set_console_color(colorize: bool, stream: HANDLE, color: CC) {
    let color = Some(color).filter(|_| colorize);
    match color {
        Some(CC::Cyan) => set_console_color_impl(stream, RAW_CYAN),
        Some(CC::Red) => set_console_color_impl(stream, RAW_RED),
        Some(CC::Green) => set_console_color_impl(stream, RAW_GREEN),
        Some(CC::LightGreen) => set_console_color_impl(stream, RAW_LIGHT_GREEN),
        Some(CC::LightMagenta) => set_console_color_impl(stream, RAW_LIGHT_MAGENTA),
        Some(CC::Yellow) => set_console_color_impl(stream, RAW_YELLOW),
        Some(CC::LightBlue) => set_console_color_impl(stream, RAW_LIGHT_BLUE),
        Some(CC::Reset) => set_console_color_impl(stream, RAW_RESET),
        None => {}
    }
}

fn set_console_color_impl(handle: HANDLE, color: WORD) {
    unsafe {
        wincon::SetConsoleTextAttribute(handle, color);
    }
}
