rust
#[derive(Copy, Clone)]
struct Foo;

fn foo(foo_ref: &Foo) {
    let foo = *foo_ref;
}

fn main() {
    println!("Hello, world!");
}
