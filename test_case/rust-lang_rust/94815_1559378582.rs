rust
fn main() {
    foo();
}

fn foo() {
    bar();
}

fn bar() {
    assert_eq!(false, true);
}
