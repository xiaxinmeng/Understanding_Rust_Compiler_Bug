rust
#![feature(decl_macro)]

fn main() {
    macro_rules! foo {
        () => {};
    }

    macro bar() {
        () => {};
    }
}
