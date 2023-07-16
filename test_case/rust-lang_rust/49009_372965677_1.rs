Rust
fn Box<T>(t: T) -> Box<T> {
    Box::new(t)
}

fn main() {
    let Box = Box(5);
    let Box = Box(Box);
    println!("{}", Box);
}
