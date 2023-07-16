 rust
use std::env;

fn first_thing(c: i32) -> i32 {
    c + 1
}

fn second_thing(c: i32) -> i32 {
    c - 1
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let op: fn(i32) -> i32 = match &args[1][..] {
        "foo" => first_thing,
        "bar" => first_thing,
        "baz" => second_thing,
        "other" => second_thing,
        _ => return,
    };
    println!("{:?}", op(11));
}

