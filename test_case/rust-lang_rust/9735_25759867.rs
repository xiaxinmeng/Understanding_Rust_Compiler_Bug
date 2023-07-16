 rust
use foo::i::A;

pub mod foo {
    pub use foo = self::i::A;

    mod i {
        pub struct A;
    }
}

fn main() {}
