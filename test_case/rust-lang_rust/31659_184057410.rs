 rust
mod foo {
    mod bar {
        pub struct Bar(pub i32);
    }
    pub use self::bar::Bar;
}

use foo::*;
struct Bar { u: () }

fn main() {
    match Bar(0) {
        Bar(x) => println!("{}", x), // prints "0"
    }
}
