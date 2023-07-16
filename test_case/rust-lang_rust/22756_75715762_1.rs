 rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Y {
    Y1,
    Y2,
}

#[derive(Debug)]
struct X {
    items : HashMap<Y, i8>,
}

fn main()
{
    println!("Hello, world!")
}
