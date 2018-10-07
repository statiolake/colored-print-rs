#[cfg(windows)]
extern crate kernel32;
#[cfg(unix)]
extern crate termion;
#[cfg(windows)]
extern crate winapi;
#[macro_use]
pub mod color;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use color;
    use color::ConsoleColor as CC;
    use color::Stream;
    #[test]
    fn common_colored_print_impl() {
        common_colored_print_impl!(true, Stream::Stdout, CC::Yellow, "Hello, world!");
        common_colored_print_impl!(
            true,
            Stream::Stdout,
            CC::LightBlue,
            "{} {} {}",
            "This",
            "is",
            "LightBlue!"
        );
    }

    #[test]
    fn common_colored_println() {
        common_colored_println! {
            true, Stream::Stdout;
            CC::Yellow, "Hello, world!";
            CC::LightBlue, "{} {} {}", "This", "is", "LightBlue!";
        }
    }

    #[test]
    fn general_colored_println() {
        colored_println! {
            true;
            CC::Yellow, "Hello, world!";
            CC::LightBlue, "{} {} {}", "This", "is", "LightBlue!";
        }
    }
}
