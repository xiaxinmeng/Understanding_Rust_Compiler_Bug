rust
use std::fmt::Display;

fn foo<'a, T: Display + 'a>(value: T) -> Box<dyn Display + 'static> {
    let result: Box<dyn Display + 'a> = Box::new(value);
    result
}
