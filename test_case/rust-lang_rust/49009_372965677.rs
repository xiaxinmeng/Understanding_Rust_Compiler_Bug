Rust
fn Box<T>(t: T) -> Box<T> {
    Box::new(t)
}

fn main() {
    let Foo = Box(5);
    let Foo = Box(Foo);
    println!("{}", Foo);
}
