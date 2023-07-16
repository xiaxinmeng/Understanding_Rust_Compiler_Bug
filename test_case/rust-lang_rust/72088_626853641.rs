rust
fn foo() {
    let x = |()| {
        #[macro_export] macro_rules! foo {}
    };
}
