rust
#![deny(elided_lifetimes_in_paths)]

#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct Bar<'a> {
    a: &'a Foo,
}

fn fun<'a>(foo: &'a Foo) {
    let bar = Bar::<'a> {a: &foo};
    println!("{:?}", bar);
}

fn main() {
    let foo = Foo { a: 1, b: 2 };
    fun(&foo);
}
