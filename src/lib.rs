pub use termcolor::Color;

use once_cell::sync::Lazy;
use std::fmt;
use std::sync::Mutex;
use termcolor::{ColorChoice, StandardStream};

static STDOUT: Lazy<Mutex<StandardStream>> =
    Lazy::new(|| Mutex::new(StandardStream::stdout(ColorChoice::Auto)));
static STDERR: Lazy<Mutex<StandardStream>> =
    Lazy::new(|| Mutex::new(StandardStream::stderr(ColorChoice::Auto)));

#[derive(Debug)]
pub struct Colored<T> {
    value: T,
    fg: Option<Color>,
    bg: Option<Color>,
}

impl<T> Colored<T> {
    pub fn plain(value: T) -> Colored<T> {
        Colored {
            value,
            fg: None,
            bg: None,
        }
    }

    pub fn fg(self, color: Option<Color>) -> Colored<T> {
        Colored { fg: color, ..self }
    }

    pub fn bg(self, color: Option<Color>) -> Colored<T> {
        Colored { bg: color, ..self }
    }

    pub fn cleared(self) -> Colored<T> {
        Colored {
            fg: None,
            bg: None,
            ..self
        }
    }
}

trait GetColored<T> {
    fn value(&self) -> &T;
    fn fg(&self) -> Option<Color>;
    fn bg(&self) -> Option<Color>;
}

// autoref based specializiation
// see also: https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md
impl<T> GetColored<T> for &T {
    fn value(&self) -> &T {
        &self
    }

    fn fg(&self) -> Option<Color> {
        None
    }

    fn bg(&self) -> Option<Color> {
        None
    }
}

impl<T> GetColored<T> for Colored<T> {
    fn value(&self) -> &T {
        &self.value
    }

    fn fg(&self) -> Option<Color> {
        self.fg
    }

    fn bg(&self) -> Option<Color> {
        self.bg
    }
}

#[macro_export]
macro_rules! colored_write {
    ($out:expr, $fmt:literal $($args:expr),*$(,)?) => {
        let out = $out;
    };
}

#[macro_export]
macro_rules! colored_print {
    ($fmt:literal $($args:tt)*) => {
        $crate::colored_write!(STDOUT.lock(), $fmt $($args)*);
    };
}

#[macro_export]
macro_rules! colored_println {
    ($fmt:literal $($args:tt)*) => {
        $crate::colored_print!(concat!($fmt, "\n"), $($args)*);
    };
}
