Rust
extern crate foo;
use foo::Foo;

fn f<F: Foo>(_f :F) {}

fn main() {
    f([0u8; 64]);
}
