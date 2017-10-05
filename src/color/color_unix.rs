use std::fmt;
use std::io::prelude::*;
use std::io;

use termion::color as cl;
use termion::color::Fg;
use super::ConsoleColor as CC;

impl fmt::Display for CC {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CC::Cyan => write!(f, "{}", CYAN),
            CC::Red => write!(f, "{}", RED),
            CC::Green => write!(f, "{}", GREEN),
            CC::LightMagenta => write!(f, "{}", LIGHT_MAGENTA),
            CC::Yellow => write!(f, "{}", YELLOW),
            CC::LightBlue => write!(f, "{}", LIGHT_BLUE),
            CC::Reset => write!(f, "{}", RESET),
        }
    }
}

const CYAN: Fg<cl::Cyan> = Fg(cl::Cyan);
const RED: Fg<cl::Red> = Fg(cl::Red);
const GREEN: Fg<cl::Green> = Fg(cl: Green);
const LIGHT_MAGENTA: Fg<cl::LightMagenta> = Fg(cl::LightMagenta);
const YELLOW: Fg<cl::Yellow> = Fg(cl::Yellow);
const LIGHT_BLUE: Fg<cl::LightBlue> = Fg(cl::LightBlue);
const RESET: Fg<cl::Reset> = Fg(cl::Reset);
