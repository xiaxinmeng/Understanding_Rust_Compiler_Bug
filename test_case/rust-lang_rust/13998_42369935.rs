 rust
pub struct Foo<T> {
    listener: fn(&T)
}

impl<T> Foo<T> {
    pub fn new(listener: fn(&T)) -> Foo<T> {
        Foo {
            listener: listener
        }
    }
}

fn hello<T>(args: T) {
    println!("hello");
}

fn main() {
    let f = Foo::new(hello);
}
