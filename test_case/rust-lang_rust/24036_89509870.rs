 rust
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let op: Box<Fn(i32) -> i32> = match &args[1][..] {
        "foo" => Box::new(|c: i32| c + 1),
        "bar" => Box::new(|c: i32| c + 1),
        "baz" => Box::new(|c: i32| c - 1),
        "other" => Box::new(|c: i32| c - 1),
        _ => return,
    };
    println!("{:?}", op(11));
}
