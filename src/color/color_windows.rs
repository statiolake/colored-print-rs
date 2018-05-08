#![macro_use]
use kernel32;
use std::fmt;
use std::io;
use std::io::prelude::*;
use winapi;
use winapi::minwindef::{DWORD, WORD};
use winapi::wincon;

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
            let handle = kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE);
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

const RAW_CYAN: DWORD = wincon::FOREGROUND_BLUE | wincon::FOREGROUND_GREEN;
const RAW_RED: DWORD = wincon::FOREGROUND_RED;
const RAW_GREEN: DWORD = wincon::FOREGROUND_GREEN;
const RAW_LIGHT_GREEN: DWORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_INTENSITY;
const RAW_LIGHT_MAGENTA: DWORD =
    wincon::FOREGROUND_BLUE | wincon::FOREGROUND_RED | wincon::FOREGROUND_INTENSITY;
const RAW_YELLOW: DWORD = wincon::FOREGROUND_GREEN | wincon::FOREGROUND_RED;
const RAW_LIGHT_BLUE: DWORD = wincon::FOREGROUND_BLUE | wincon::FOREGROUND_INTENSITY;
const RAW_RESET: DWORD =
    wincon::FOREGROUND_GREEN | wincon::FOREGROUND_BLUE | wincon::FOREGROUND_RED;

const CYAN: ConsoleAttribute = ConsoleAttribute::Attr(RAW_CYAN);
const RED: ConsoleAttribute = ConsoleAttribute::Attr(RAW_RED);
const GREEN: ConsoleAttribute = ConsoleAttribute::Attr(RAW_GREEN);
const LIGHT_GREEN: ConsoleAttribute = ConsoleAttribute::Attr(RAW_LIGHT_GREEN);
const LIGHT_MAGENTA: ConsoleAttribute = ConsoleAttribute::Attr(RAW_LIGHT_MAGENTA);
const YELLOW: ConsoleAttribute = ConsoleAttribute::Attr(RAW_YELLOW);
const LIGHT_BLUE: ConsoleAttribute = ConsoleAttribute::Attr(RAW_LIGHT_BLUE);
const RESET: ConsoleAttribute = ConsoleAttribute::Reset;
