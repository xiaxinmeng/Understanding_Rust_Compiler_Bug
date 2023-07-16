 rust
trait Foo {
    fn foo() -> i32;
}

struct Bar;

impl Foo for Bar {
    fn foo() -> i32 {
        10
    }
}

fn main() {
    println!("{}", <Bar as Foo>::foo());
}
