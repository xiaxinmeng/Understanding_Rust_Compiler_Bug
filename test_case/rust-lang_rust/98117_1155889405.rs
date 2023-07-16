rs
use std::fmt::Display;

trait Static: 'static {}
impl<T> Static for &'static T {}

fn foo<T: Display>(x: T) -> Box<dyn Display>
where
    &'static T: Static,
{
    Box::new(x)
}

fn main() {
    let s = foo(&String::from("blah blah blah"));
    println!("{s}");
}
