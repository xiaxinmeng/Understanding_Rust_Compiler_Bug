Rust
mod foo {
    pub fn Box<T>(t: T) -> Box<T> {
        Box::new(t)
    }
}

fn main() {
    let Box = foo::Box(5);
    let Box = foo::Box(Box);
    println!("{}", Box);
}
