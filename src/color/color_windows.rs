use winapi::shared::minwindef::WORD;
use winapi::um::winnt::HANDLE;
use winapi::um::{processenv, winbase, wincon};

use std::io;
use std::io::prelude::*;

use super::ConsoleColor as CC;
use Stream;

fn raw_fg_color(red: bool, green: bool, blue: bool, int: bool) -> WORD {
    let red = if red { wincon::FOREGROUND_RED } else { 0 };
    let green = if green { wincon::FOREGROUND_GREEN } else { 0 };
    let blue = if blue { wincon::FOREGROUND_BLUE } else { 0 };
    let int = if int { wincon::FOREGROUND_INTENSITY } else { 0 };

    red | green | blue | int
}

fn fg_cyan() -> WORD {
    raw_fg_color(false, true, true, false)
}

fn fg_red() -> WORD {
    raw_fg_color(true, false, false, false)
}

fn fg_green() -> WORD {
    raw_fg_color(false, true, false, false)
}

fn fg_light_green() -> WORD {
    raw_fg_color(false, true, false, true)
}

fn fg_light_magenta() -> WORD {
    raw_fg_color(true, false, true, true)
}

fn fg_yellow() -> WORD {
    raw_fg_color(true, true, false, false)
}

fn fg_light_blue() -> WORD {
    raw_fg_color(false, false, true, true)
}

fn fg_reset() -> WORD {
    // temporary implementation
    raw_fg_color(true, true, true, true)
}

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
        Some(CC::Cyan) => set_console_color_impl(stream, fg_cyan()),
        Some(CC::Red) => set_console_color_impl(stream, fg_red()),
        Some(CC::Green) => set_console_color_impl(stream, fg_green()),
        Some(CC::LightGreen) => set_console_color_impl(stream, fg_light_green()),
        Some(CC::LightMagenta) => set_console_color_impl(stream, fg_light_magenta()),
        Some(CC::Yellow) => set_console_color_impl(stream, fg_yellow()),
        Some(CC::LightBlue) => set_console_color_impl(stream, fg_light_blue()),
        Some(CC::Reset) => set_console_color_impl(stream, fg_reset()),
        None => {}
    }
}

fn set_console_color_impl(handle: HANDLE, color: WORD) {
    unsafe {
        wincon::SetConsoleTextAttribute(handle, color);
    }
}
