rust
// bar/src/main.rs
extern crate foo;
extern crate tokio_core;

fn main() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    core.run(foo::Foo::new()).unwrap();
}
