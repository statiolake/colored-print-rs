pub mod color;

#[macro_export]
macro_rules! common_colored_print_impl {
    ($colorize:expr, $stream:expr, $color:expr, $fmt:expr $(,$args:expr)*) => {
        $crate::color::print($colorize, $stream, $color, format!($fmt $(,$args)*));
    };
}

#[macro_export]
macro_rules! common_colored_print {
    ($colorize:expr, $stream:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        $(
            $crate::common_colored_print_impl!($colorize, $stream, $color, $fmt $(,$args)*);
        )*
        $crate::color::print($colorize, $stream, $crate::color::ConsoleColor::Reset, "");
    };
}

#[macro_export]
macro_rules! common_colored_println {
    ($colorize:expr, $stream:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        $crate::common_colored_print!($colorize, $stream; $($color, $fmt $(,$args)*;)*);
        $crate::color::print($colorize, $stream, $crate::color::ConsoleColor::Reset, "\n");
    };
}

#[macro_export]
macro_rules! colored_print {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        $crate::common_colored_print!($colorize, $crate::Stream::Stdout; $($color, $fmt $(,$args)*;)*);
    };
}

#[macro_export]
macro_rules! colored_println {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        $crate::common_colored_println!($colorize, $crate::Stream::Stdout; $($color, $fmt $(,$args)*;)*);
    };
}

#[macro_export]
macro_rules! colored_eprint {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        $crate::common_colored_print!($colorize, $crate::Stream::Stderr; $($color, $fmt $(,$args)*;)*);
    };
}

#[macro_export]
macro_rules! colored_eprintln {
    ($colorize:expr; $($color:expr, $fmt:expr $(,$args:expr)*;)+) => {
        $crate::common_colored_println!($colorize, $crate::Stream::Stderr; $($color, $fmt $(,$args)*;)*);
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Stream {
    Stdout,
    Stderr,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use crate::color::ConsoleColor as CC;
    use crate::Stream;
    use crate::{colored_eprintln, colored_println};
    #[test]
    fn it_works() {
        colored_println! {
            true;
            CC::LightBlue, "Hello, ";
            CC::LightGreen, "this is ";
            CC::Yellow, "stdout";
        }

        colored_eprintln! {
            true;
            CC::LightBlue, "Hello, ";
            CC::LightGreen, "this is ";
            CC::Yellow, "stderr";
        }

        common_colored_println! {
            true, Stream::Stdout;
            CC::LightBlue, "Hello, ";
            CC::LightGreen, "this is ";
            CC::Red, "alternative ";
            CC::Yellow, "stdout";
        }

        common_colored_println! {
            true, Stream::Stderr;
            CC::LightBlue, "Hello, ";
            CC::LightGreen, "this is ";
            CC::Red, "alternative ";
            CC::Yellow, "stderr";
        }
    }
}
