 rust
trait Foo { }

fn foo<T: Foo>(x: T) { }

fn main() {
    let x: &Fn(&i32) = &foo;
}
