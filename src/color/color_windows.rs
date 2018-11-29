use lazy_static::lazy_static;
use winapi::shared::minwindef::WORD;
use winapi::um::winnt::HANDLE;
use winapi::um::{processenv, winbase, wincon};

use std::io;
use std::io::prelude::*;
use std::mem;

use super::ConsoleColor as CC;
use crate::Stream;

lazy_static! {
    static ref STDOUT_DEFAULT_ATTRIBUTE: WORD = get_attributes_for(Stream::Stdout);
    static ref STDERR_DEFAULT_ATTRIBUTE: WORD = get_attributes_for(Stream::Stderr);
    static ref STDOUT_DEFAULT_BACKGROUND_ATTRIBUTE: WORD =
        *STDOUT_DEFAULT_ATTRIBUTE & !raw_fg_color(true, true, true, true);
    static ref STDERR_DEFAULT_BACKGROUND_ATTRIBUTE: WORD =
        *STDERR_DEFAULT_ATTRIBUTE & !raw_fg_color(true, true, true, true);
}

fn get_attributes_for(stream: Stream) -> WORD {
    unsafe {
        let mut csbi: wincon::CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
        wincon::GetConsoleScreenBufferInfo(get_stream_handle_for(stream), &mut csbi);
        csbi.wAttributes
    }
}

fn get_stream_handle_for(stream: Stream) -> HANDLE {
    let raw_stream = match stream {
        Stream::Stdout => winbase::STD_OUTPUT_HANDLE,
        Stream::Stderr => winbase::STD_ERROR_HANDLE,
    };
    unsafe { processenv::GetStdHandle(raw_stream) }
}

fn raw_fg_color(red: bool, green: bool, blue: bool, int: bool) -> WORD {
    let red = if red { wincon::FOREGROUND_RED } else { 0 };
    let green = if green { wincon::FOREGROUND_GREEN } else { 0 };
    let blue = if blue { wincon::FOREGROUND_BLUE } else { 0 };
    let int = if int { wincon::FOREGROUND_INTENSITY } else { 0 };

    red | green | blue | int
}

fn bg_default(stream: Stream) -> WORD {
    match stream {
        Stream::Stdout => *STDOUT_DEFAULT_BACKGROUND_ATTRIBUTE,
        Stream::Stderr => *STDERR_DEFAULT_BACKGROUND_ATTRIBUTE,
    }
}

fn fg_cyan(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(false, true, true, false)
}

fn fg_red(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(true, false, false, false)
}

fn fg_green(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(false, true, false, false)
}

fn fg_light_green(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(false, true, false, true)
}

fn fg_light_magenta(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(true, false, true, true)
}

fn fg_yellow(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(true, true, false, false)
}

fn fg_light_blue(stream: Stream) -> WORD {
    bg_default(stream) | raw_fg_color(false, false, true, true)
}

fn fg_reset(stream: Stream) -> WORD {
    match stream {
        Stream::Stdout => *STDOUT_DEFAULT_ATTRIBUTE,
        Stream::Stderr => *STDERR_DEFAULT_ATTRIBUTE,
    }
}

pub fn print<S: AsRef<str>>(colorize: bool, stream: Stream, color: CC, body: S) {
    io::stdout().flush().unwrap();
    io::stderr().flush().unwrap();
    set_console_color(colorize, stream, color);
    match stream {
        Stream::Stdout => write!(io::stdout(), "{}", body.as_ref()).unwrap(),
        Stream::Stderr => write!(io::stderr(), "{}", body.as_ref()).unwrap(),
    }
    io::stdout().flush().unwrap();
    io::stderr().flush().unwrap();
    set_console_color(colorize, stream, CC::Reset);
}

fn set_console_color(colorize: bool, stream: Stream, color: CC) {
    let color = Some(color).filter(|_| colorize);
    match color {
        Some(CC::Cyan) => set_console_color_impl(stream, fg_cyan(stream)),
        Some(CC::Red) => set_console_color_impl(stream, fg_red(stream)),
        Some(CC::Green) => set_console_color_impl(stream, fg_green(stream)),
        Some(CC::LightGreen) => set_console_color_impl(stream, fg_light_green(stream)),
        Some(CC::LightMagenta) => set_console_color_impl(stream, fg_light_magenta(stream)),
        Some(CC::Yellow) => set_console_color_impl(stream, fg_yellow(stream)),
        Some(CC::LightBlue) => set_console_color_impl(stream, fg_light_blue(stream)),
        Some(CC::Reset) => set_console_color_impl(stream, fg_reset(stream)),
        None => {}
    }
}

fn set_console_color_impl(stream: Stream, color: WORD) {
    unsafe {
        wincon::SetConsoleTextAttribute(get_stream_handle_for(stream), color);
    }
}
