rust
use std::fmt::Display;

trait Static: 'static {}
impl<T> Static for &'static T {}

fn foo<S: Display>(x: S) -> Box<dyn Display>
where
    &'static S: Static,
{
    Box::new(x)
}

fn main() {
    let s = foo(&String::from("blah blah blah"));
    println!("{}", s);
}
