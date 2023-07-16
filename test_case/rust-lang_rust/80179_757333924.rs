rust
#![feature(generators)]

fn foo() -> _ {
    || yield i32
}
