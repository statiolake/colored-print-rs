use std::io;
use std::io::Write;

use super::ConsoleColor as CC;
use termion::color as cl;
use termion::color::Fg;
use Stream;

const CYAN: Fg<cl::Cyan> = Fg(cl::Cyan);
const RED: Fg<cl::Red> = Fg(cl::Red);
const GREEN: Fg<cl::Green> = Fg(cl::Green);
const LIGHT_GREEN: Fg<cl::LightGreen> = Fg(cl::LightGreen);
const LIGHT_MAGENTA: Fg<cl::LightMagenta> = Fg(cl::LightMagenta);
const YELLOW: Fg<cl::Yellow> = Fg(cl::Yellow);
const LIGHT_BLUE: Fg<cl::LightBlue> = Fg(cl::LightBlue);
const RESET: Fg<cl::Reset> = Fg(cl::Reset);

pub fn print(colorize: bool, stream: Stream, color: CC, body: &str) {
    match stream {
        Stream::Stdout => print_impl(colorize, &mut io::stdout(), color, body),
        Stream::Stderr => print_impl(colorize, &mut io::stderr(), color, body),
    }
}

fn print_impl<W: Write>(colorize: bool, stream: &mut W, color: CC, body: &str) {
    print_color_sequence(colorize, stream, color);
    write!(stream, "{}", body);
    print_color_sequence(colorize, stream, CC::Reset);
}

fn print_color_sequence<W: Write>(colorize: bool, stream: &mut W, color: CC) {
    let color = Some(color).filter(|_| colorize);
    match color {
        Some(CC::Cyan) => write!(stream, "{}", CYAN).unwrap(),
        Some(CC::Red) => write!(stream, "{}", RED).unwrap(),
        Some(CC::Green) => write!(stream, "{}", GREEN).unwrap(),
        Some(CC::LightGreen) => write!(stream, "{}", LIGHT_GREEN).unwrap(),
        Some(CC::LightMagenta) => write!(stream, "{}", LIGHT_MAGENTA).unwrap(),
        Some(CC::Yellow) => write!(stream, "{}", YELLOW).unwrap(),
        Some(CC::LightBlue) => write!(stream, "{}", LIGHT_BLUE).unwrap(),
        Some(CC::Reset) => write!(stream, "{}", RESET).unwrap(),
        None => {}
    }
}
