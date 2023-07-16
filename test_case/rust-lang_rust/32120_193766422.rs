 rust
mod T {
    pub fn f() { println!("foo"); }
}

trait Bar {
    fn f() { println!("bar"); }
}
impl Bar for () {}

fn f<T: Bar>(t: T) {
    T::f();
}

fn main() {
    f(()) // This prints "foo". If the module `T` is removed, it prints "bar".
}
