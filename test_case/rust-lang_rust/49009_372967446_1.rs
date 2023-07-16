Rust
mod foo {
    pub fn Box<T>(t: T) -> Box<T> {
        Box::new(t)
    }
}

use foo::Box;

fn main() {
    let Box = Box(5);
    let Box = Box(Box);
    println!("{}", Box);
}
