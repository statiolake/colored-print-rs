# colored_print: the library to colorize terminal output for rust.

## supported environments

- Windows
- Linux

## How to use

add below lines to your Cargo.toml:

```toml
[dependencies.colored_print]
git = "https://github.com/statiolake/colored_print.git"
```

then you can use

```rust
#[macro_use]
extern crate colored_print;

fn main() {
    use colored_print::color::ConsoleColor::{Yellow, LightBlue};

    // prints *one* line that consists of yellow sentence and light-blue one.
    colored_println! {
        true; // whether colorizes or does not; if false, output will not be colorized.
        Yellow, "Hello, this is yellow text! there is no linebreak here ->",; // if there is no placeholders, the last comma is a bit ugly.
        LightBlue, "<-. You can also use placeholders like {}", "this";
    }
    // ... and terminal color will automatically be reset.
    println!("this line will be printed with the default color.");
}
```
