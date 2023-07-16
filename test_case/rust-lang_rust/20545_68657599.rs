 rust
use std::any::Any;
use std::boxed::BoxAny;

fn main() {
    let x: Box<Any> = box "rust";
    match x.downcast::<&'static str>() {
        Ok(s) => println!("str: {}", s),
        Err(x) => println!("{}", x)
    }
}
