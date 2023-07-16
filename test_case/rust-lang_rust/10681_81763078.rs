 rust
macro_rules! foo (
    () => (
        struct Foo;
        struct Bar;
    )
);

fn main() {
    foo!();
}
