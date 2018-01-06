#[cfg(unix)]
extern crate termion;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate kernel32;
#[macro_use]
pub mod color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        colored_println! {
            true;
            ::color::ConsoleColor::Yellow, "Hello, world!";
            ::color::ConsoleColor::LightBlue, "{} {} {}", "This", "is", "LightBlue!";
        }
        assert_eq!(2 + 2, 4);
    }
}
