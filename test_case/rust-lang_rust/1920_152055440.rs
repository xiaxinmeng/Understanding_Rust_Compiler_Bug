 rust
mod foo {
    extern crate core;
}

fn assert_clone<T>() where T : Clone { }

fn main() {
    assert_clone::<foo::core::sync::atomic::AtomicBool>();
}
