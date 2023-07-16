 rust
fn foo() -> bool {
    let a = thread::panicking();
    bar();
    a && thread::panicking()
}
