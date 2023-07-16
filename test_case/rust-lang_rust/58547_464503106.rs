rust
trait X {}
trait Y {
    const MY_CONST: X; // X is a trait! Compiler does not complain ...
}

struct Foo;
#[derive(Debug)]
struct Bar;

impl Y for Foo {
    // We implement MY_CONST as Bar that does not implement X.
    // Again the compiler does not complain ...
    const MY_CONST: X = Bar;
}

fn main() {
    println!("{:?}", Foo::MY_CONST);
}
