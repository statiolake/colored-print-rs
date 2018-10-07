use kernel32;
use winapi::shared::minwindef::{DWORD, WORD};
use winapi::um::{winbase, wincon};

use std::fmt;
use std::io;
use std::io::prelude::*;

use super::ConsoleColor as CC;

impl fmt::Display for CC {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CC::Cyan => write!(f, "{}", CYAN),
            CC::Red => write!(f, "{}", RED),
            CC::Green => write!(f, "{}", GREEN),
            CC::LightGreen => write!(f, "{}", LIGHT_GREEN),
            CC::LightMagenta => write!(f, "{}", LIGHT_MAGENTA),
            CC::Yellow => write!(f, "{}", YELLOW),
            CC::LightBlue => write!(f, "{}", LIGHT_BLUE),
            CC::Reset => write!(f, "{}", RESET),
        }
    }
}

pub enum ConsoleAttribute {
    Attr(DWORD),
    Reset,
}

impl fmt::Display for ConsoleAttribute {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        io::stdout().flush().unwrap();
        io::stderr().flush().unwrap();
        unsafe {
            let handle = kernel32::GetStdHandle(winbase::STD_OUTPUT_HANDLE);
            match *self {
                ConsoleAttribute::Attr(attr) => {
                    kernel32::SetConsoleTextAttribute(handle, attr as WORD);
                }
                _ => {
                    kernel32::SetConsoleTextAttribute(handle, RAW_RESET as WORD);
                }
            }
        }
        Ok(())
    }
}

const RAW_CYAN: WORD = wincon::FOREGROUND_BLUE | wincon::FOREGROUND_GREEN;
const RAW_RED: WORD = wincon::FOREGROUND_RED;
const RAW_GREEN: WORD = wincon::FOREGROUND_GREEN;
const RAW_LIGHT_GREEN: WORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_INTENSITY;
const RAW_LIGHT_MAGENTA: WORD =
    wincon::FOREGROUND_BLUE | wincon::FOREGROUND_RED | wincon::FOREGROUND_INTENSITY;
const RAW_YELLOW: WORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_RED;
const RAW_LIGHT_BLUE: WORD = wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY;
const RAW_RESET: WORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE | wincon::FOREGROUND_RED;

const CYAN: ConsoleAttribute = ConsoleAttribute::Attr(RAW_CYAN as DWORD);
const RED: ConsoleAttribute = ConsoleAttribute::Attr(RAW_RED as DWORD);
const GREEN: ConsoleAttribute = ConsoleAttribute::Attr(RAW_GREEN as DWORD);
const LIGHT_GREEN: ConsoleAttribute = ConsoleAttribute::Attr(RAW_LIGHT_GREEN as DWORD);
const LIGHT_MAGENTA: ConsoleAttribute = ConsoleAttribute::Attr(RAW_LIGHT_MAGENTA as DWORD);
const YELLOW: ConsoleAttribute = ConsoleAttribute::Attr(RAW_YELLOW as DWORD);
const LIGHT_BLUE: ConsoleAttribute = ConsoleAttribute::Attr(RAW_LIGHT_BLUE as DWORD);
const RESET: ConsoleAttribute = ConsoleAttribute::Reset;
