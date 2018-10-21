use winapi::shared::minwindef::WORD;
use winapi::um::winnt::HANDLE;
use winapi::um::{processenv, winbase, wincon};

use std::io;
use std::io::prelude::*;
use std::mem;

use super::ConsoleColor as CC;
use Stream;

lazy_static! {
    static ref STDOUT_DEFAULT_ATTRIBUTE: WORD = get_attributes_for(winbase::STD_OUTPUT_HANDLE);
    static ref STDERR_DEFAULT_ATTRIBUTE: WORD = get_attributes_for(winbase::STD_ERROR_HANDLE);
    static ref STDOUT_DEFAULT_BACKGROUND_ATTRIBUTE: WORD =
        *STDOUT_DEFAULT_ATTRIBUTE & !raw_fg_color(true, true, true, true);
    static ref STDERR_DEFAULT_BACKGROUND_ATTRIBUTE: WORD =
        *STDERR_DEFAULT_ATTRIBUTE & !raw_fg_color(true, true, true, true);
}

fn get_attributes_for(stream_type: u32) -> WORD {
    let csbi = unsafe {
        let mut csbi: wincon::CONSOLE_SCREEN_BUFFER_INFO = mem::zeroed();
        wincon::GetConsoleScreenBufferInfo(processenv::GetStdHandle(stream_type), &mut csbi);
        csbi
    };
    csbi.wAttributes
}

fn raw_fg_color(red: bool, green: bool, blue: bool, int: bool) -> WORD {
    let red = if red { wincon::FOREGROUND_RED } else { 0 };
    let green = if green { wincon::FOREGROUND_GREEN } else { 0 };
    let blue = if blue { wincon::FOREGROUND_BLUE } else { 0 };
    let int = if int { wincon::FOREGROUND_INTENSITY } else { 0 };

    red | green | blue | int
}

fn bg_default(stream: HANDLE) -> WORD {
    unsafe {
        if stream == processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE) {
            *STDOUT_DEFAULT_BACKGROUND_ATTRIBUTE
        } else if stream == processenv::GetStdHandle(winbase::STD_ERROR_HANDLE) {
            *STDERR_DEFAULT_BACKGROUND_ATTRIBUTE
        } else {
            unreachable!();
        }
    }
}

fn fg_cyan(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(false, true, true, false)
}

fn fg_red(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(true, false, false, false)
}

fn fg_green(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(false, true, false, false)
}

fn fg_light_green(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(false, true, false, true)
}

fn fg_light_magenta(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(true, false, true, true)
}

fn fg_yellow(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(true, true, false, false)
}

fn fg_light_blue(stream: HANDLE) -> WORD {
    bg_default(stream) | raw_fg_color(false, false, true, true)
}

fn fg_reset(stream: HANDLE) -> WORD {
    unsafe {
        if stream == processenv::GetStdHandle(winbase::STD_OUTPUT_HANDLE) {
            *STDOUT_DEFAULT_ATTRIBUTE
        } else if stream == processenv::GetStdHandle(winbase::STD_ERROR_HANDLE) {
            *STDERR_DEFAULT_ATTRIBUTE
        } else {
            unreachable!();
        }
    }
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

fn set_console_color_impl(handle: HANDLE, color: WORD) {
    unsafe {
        wincon::SetConsoleTextAttribute(handle, color);
    }
}
