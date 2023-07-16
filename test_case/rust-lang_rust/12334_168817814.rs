 rust
mod bar {
    pub trait foo {}

    fn foo() {}
}

use bar::foo;

fn main() {
    foo();
}
