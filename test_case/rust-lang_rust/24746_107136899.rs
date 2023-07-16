 rust
#![feature(rustdoc)]

extern crate rustdoc;

use rustdoc::html::markdown::Markdown;

fn main() {
    let source = "`This` should not be repeated. After `This<U>` nothing should be underlined.";

    let html = Markdown(source);

    println!("{}", html);
}
