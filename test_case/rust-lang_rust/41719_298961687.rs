rust
#![feature(use_extern_macros)]

fn main() {
    enum Foo {}
    let _ = Foo::bar!();
}
