rust
macro_rules! empty {
    () => { }
}

fn foo() -> bool {
    { true }
    empty!();
}
