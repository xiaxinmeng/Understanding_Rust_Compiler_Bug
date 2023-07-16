 rust
fn foo() {}

macro_rules! invoke_foo(
    () => (foo())
);

fn main() {
    fn foo() { panic!(":("); }
    invoke_foo!();
}
