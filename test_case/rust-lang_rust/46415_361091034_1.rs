rust
#![feature(conservative_impl_trait)]

fn foo() -> impl Fn() {
    let t = foo();
    move || {
        t();
    }
}

fn main() {
    (foo())();
}
