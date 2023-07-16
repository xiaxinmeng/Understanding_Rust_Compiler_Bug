rust
#[derive(Debug)]
struct Foo {
    a: i32,
    b: i32,
}

fn main() {
    let mut foo: Foo; // `foo` is not initialized, we're just giving a type hint
    foo.a = 1; // no error
    foo.b = 2; // no error
    println!("{}", foo.a); // error[E0381]: use of possibly uninitialized variable: `foo.a`
    println!("{:?}", foo); // error[E0381]: use of possibly uninitialized variable: `foo`
}
