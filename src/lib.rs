#[cfg(windows)]
extern crate kernel32;
#[cfg(unix)]
extern crate termion;
#[cfg(windows)]
extern crate winapi;
#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod color;

#[derive(Debug, Copy, Clone)]
pub enum Stream {
    Stdout,
    Stderr,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use color::ConsoleColor as CC;
    use Stream;
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
