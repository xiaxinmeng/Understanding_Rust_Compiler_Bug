 rust
pub mod foo {
    pub struct Bar(i32);
    impl Bar {
        pub fn new(x: i32) -> Bar { Bar(x) }
    }
}

use foo::Bar;

fn main() {
    let x = Bar::new(1);
    let Bar(y) = x;
}
