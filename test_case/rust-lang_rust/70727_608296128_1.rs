rust
#![deny(unconditional_recursion)]
fn bar() {
    foo();
}

fn foo() {
    bar();
}

fn main() {
    foo();
}
